//! Types for representing and maintaining a lexical scope during AST traversal.

use crate::{
    model::{
        schema::Table,
        sources::{ColumnRef, Relation, Source, TableColumn},
    },
    resolution_error::{InvariantFailedError, ResolutionError},
    sources::SourceItem,
};
use sqlparser::ast::Ident;

use core::ops::{Deref, DerefMut};
use std::{collections::BTreeSet, mem};

/// A stack of [`Scope`] structs.
///
/// Implements `Deref<Target = Scope>` where `Scope` is the top of the stack
/// (current lexical scope).
#[derive(Debug, PartialEq, Eq, Default)]
pub struct ScopeStack {
    top: Scope,
}

impl Deref for ScopeStack {
    type Target = Scope;

    fn deref(&self) -> &Self::Target {
        &self.top
    }
}

impl DerefMut for ScopeStack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.top
    }
}

impl ScopeStack {
    /// Push a new empty [`Scope`] onto the stack.
    pub(crate) fn push(&mut self) {
        let top = mem::take(&mut self.top);
        self.top = Scope {
            bindings: BTreeSet::default(),
            parent: Some(Box::new(top)),
        }
    }

    /// Pop the top [`Scope`] of the stack.
    ///
    /// Will panic if an attempt is made to pop the `root` [`Scope`].
    pub(crate) fn pop(&mut self) {
        let mut me = mem::take(self);
        match me.top {
            Scope {
                bindings: _,
                parent: None,
            } => panic!("Cannot pop the root scope"),
            Scope {
                bindings: _,
                parent: Some(parent),
            } => me.top = *parent,
        }
        *self = me
    }

    /// Resets (clears) the stack.
    pub(crate) fn reset(&mut self) {
        self.top = Scope::default()
    }
}

/// A lexical scope.
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Scope {
    bindings: BTreeSet<Relation>,
    parent: Option<Box<Scope>>,
}

impl Scope {
    /// Expand usage of a wildcard into the actual concrete table-columns it stands for.
    pub fn resolve_wildcard(&self) -> Result<Vec<Source>, ResolutionError> {
        if self.bindings.is_empty() {
            Err(ResolutionError::InvariantFailed(
                InvariantFailedError::EmptyScope,
            ))
        } else {
            let resolved: Vec<Source> = self
                .bindings
                .iter()
                .flat_map(|binding| match binding {
                    Relation::TableLike(Table { columns, name }, _alias) => columns
                        .iter()
                        .map(|c| {
                            Source::single(SourceItem::TableColumn(TableColumn {
                                table: name.clone(),
                                column: ColumnRef::Identifier(c.to_string()),
                            }))
                        })
                        .collect::<Vec<_>>(),
                    Relation::SubQuery(_subquery, _columns, _alias) => unimplemented!(),
                })
                .collect();

            Ok(resolved)
        }
    }

    /// Expand usage of a qualified wildcard into the actual concrete table-columns it stands for.
    pub fn resolve_qualified_wildcard(
        &self,
        idents: &[Ident],
    ) -> Result<Vec<Source>, ResolutionError> {
        if idents.len() > 1 {
            unimplemented!()
        }
        if self.bindings.is_empty() {
            Err(ResolutionError::InvariantFailed(
                InvariantFailedError::EmptyScope,
            ))
        } else {
            let resolved: Option<Vec<Source>> =
                self.bindings.iter().find_map(|binding| match binding {
                    Relation::TableLike(Table { columns, name }, alias) => {
                        if *name == idents[0].to_string() || Some(name.clone()) == *alias {
                            Some(
                                columns
                                    .iter()
                                    .map(|c| {
                                        Source::single(SourceItem::TableColumn(TableColumn {
                                            table: name.clone(),
                                            column: ColumnRef::Identifier(c.to_string()),
                                        }))
                                    })
                                    .collect::<Vec<_>>(),
                            )
                        } else {
                            None
                        }
                    }
                    Relation::SubQuery(_subquery, _columns, _alias) => unimplemented!(),
                });

            resolved.ok_or(ResolutionError::NoSuchIdentifier(idents[0].to_string()))
        }
    }

    /// Resolves usage of an identifier.
    // TODO: needs to be more general and resolve by ordinal too.
    pub fn resolve_ident(&self, column_name: &Ident) -> Result<Source, ResolutionError> {
        self.bindings
            .iter()
            .find_map(|binding| match binding {
                Relation::TableLike(table @ Table { .. }, _alias) => {
                    if table.contains_column(&column_name.to_string()) {
                        Some(Source::single(SourceItem::TableColumn(TableColumn::new(
                            table.name.clone(),
                            ColumnRef::Identifier(column_name.to_string()),
                        ))))
                    } else {
                        None
                    }
                }
                Relation::SubQuery(_query, projection_sources, _alias) => projection_sources
                    .iter()
                    .find_map(|(col_ref, source)| match col_ref {
                        ColumnRef::Identifier(id) => {
                            if id == &column_name.to_string() {
                                Some(source.clone())
                            } else {
                                None
                            }
                        }
                        ColumnRef::Ordinal(_) => todo!(),
                    }),
            })
            .ok_or(ResolutionError::NoSuchIdentifier(column_name.to_string()))
    }

    /// Resolves usage of a compound identifier.
    ///
    /// Note that currently only compound identifier of length 2 are supported
    /// and resolution will fail if the identifier has more than two parts.
    pub fn resolve_compound_ident(&self, idents: &[Ident]) -> Result<Source, ResolutionError> {
        // TODO: deal with multiple schemas (idents.len() > 2). Currently defaulting implicitly to the public schema within a database.
        // TODO: change type from Vec (which is unbounded) to an enum with a fixed number of variants
        // TODO: take into account the `search_path` (PG-specific, but suggests the resolution logic should be behind a trait which is implemented per database variant)
        if idents.len() != 2 {
            return Err(ResolutionError::InvariantFailed(
                InvariantFailedError::MaxCompoundIdentLengthExceeded(idents.len() as u8),
            ));
        }
        let first_part = idents[0].to_string();
        let second_part = idents[1].to_string();

        self.bindings
            .iter()
            .find_map(|relation| match relation {
                Relation::TableLike(table, alias) => {
                    if *alias == Some(first_part.clone()) || table.name == first_part {
                        table
                            .columns
                            .iter()
                            .find(|c| c.name == second_part)
                            .map(|_| {
                                Source::single(SourceItem::TableColumn(TableColumn {
                                    table: table.name.clone(),
                                    column: ColumnRef::Identifier(second_part.to_string()),
                                }))
                            })
                    } else {
                        None
                    }
                }
                Relation::SubQuery(_, sources, Some(alias)) => {
                    eprintln!("first_part: {first_part}");
                    eprintln!("second_part: {second_part}");
                    eprintln!("sources {:#?}", sources);
                    if *alias == first_part {
                        sources.iter().find_map(|(col_ref, source)| {
                            if col_ref == &ColumnRef::Identifier(second_part.clone()) {
                                Some(source.clone())
                            } else {
                                None
                            }
                        })
                    } else {
                        None
                    }
                }
                Relation::SubQuery(_, _, None) => None,
            })
            .ok_or(ResolutionError::NoSuchCompoundIdentifier(format!(
                "{}.{}",
                idents[0], idents[1]
            )))
    }

    /// Add a table/view/subquery to the current scope.
    pub fn add_relation(&mut self, relation: Relation) -> Result<Relation, ResolutionError> {
        if !self.bindings.contains(&relation) {
            self.bindings.insert(relation.clone());
            Ok(relation.clone())
        } else {
            match relation {
                Relation::TableLike(table, alias) => Err(ResolutionError::AlreadyInScope(format!(
                    "Table {} (as {})",
                    &table.name,
                    &alias.unwrap_or("UNALIASED".to_owned())
                ))),
                Relation::SubQuery(subquery, _, alias) => {
                    Err(ResolutionError::AlreadyInScope(format!(
                        "SubQuery {} (as {})",
                        &subquery.to_string(),
                        &alias.unwrap_or("UNALIASED".to_owned())
                    )))
                }
            }
        }
    }
}
