//! Types for representing and maintaining a lexical scope during AST traversal.

use crate::{
    model::source_annotation::{NamedRelation, SourceAnnotation},
    projection_annotation::Projection,
    resolution_error::{InvariantFailedError, ResolutionError},
};
use sqlparser::ast::Ident;
use unicase::UniCase;

use core::ops::{Deref, DerefMut};
use std::{mem, rc::Rc};

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
            bindings: Vec::default(),
            depth: top.depth + 1,
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
                ..
            } => panic!("Cannot pop the root scope"),
            Scope {
                bindings: _,
                parent: Some(parent),
                ..
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
    bindings: Vec<NamedRelation>,
    parent: Option<Box<Scope>>,
    depth: u32,
}

impl Scope {
    /// Expand usage of a wildcard into the actual concrete table-columns it stands for.
    pub fn resolve_wildcard(&self) -> Result<Projection, ResolutionError> {
        if self.bindings.is_empty() {
            match &self.parent {
                Some(parent) => parent.resolve_wildcard(),
                None => Err(ResolutionError::InvariantFailed(
                    InvariantFailedError::EmptyScope,
                )),
            }
        } else {
            Ok(self
                .bindings
                .iter()
                .flat_map(|relation| relation.projection.columns.clone())
                .collect())
        }
    }

    /// Expand usage of a qualified wildcard into the actual concrete table-columns it stands for.
    pub fn resolve_qualified_wildcard(
        &self,
        idents: &[Ident],
    ) -> Result<Projection, ResolutionError> {
        if idents.len() > 1 {
            return Err(ResolutionError::UnsupportedCompoundIdentifierLength(
                idents.into(),
            ));
        }
        if self.bindings.is_empty() {
            match &self.parent {
                Some(parent) => return parent.resolve_qualified_wildcard(idents),
                None => {
                    return Err(ResolutionError::InvariantFailed(
                        InvariantFailedError::EmptyScope,
                    ))
                }
            }
        }
        let first_ident = idents.first().unwrap().to_string();
        let case_insensitive_ident = UniCase::new(first_ident.clone());
        let resolved: Option<Projection> = self.bindings.iter().find_map(|relation| {
            if relation.name == case_insensitive_ident {
                Some(relation.projection.clone())
            } else {
                None
            }
        });

        resolved.ok_or(ResolutionError::NoSuchIdentifier(first_ident))
    }

    /// Resolves usage of an identifier.
    pub fn resolve_ident(
        &self,
        column_name: &Ident,
    ) -> Result<Rc<SourceAnnotation>, ResolutionError> {
        let case_insensitive_ident = UniCase::new(column_name.to_string());
        self.bindings
            .iter()
            .find_map(|relation| {
                relation
                    .projection
                    .columns
                    .iter()
                    .find_map(|(source_annotation, column)| {
                        if Some(true) == column.as_ref().map(|c| c == &case_insensitive_ident) {
                            Some(source_annotation.clone())
                        } else {
                            None
                        }
                    })
            })
            .or_else(|| match &self.parent {
                Some(parent) => parent.resolve_ident(column_name).ok(),
                None => None,
            })
            .ok_or(ResolutionError::NoSuchIdentifier(column_name.to_string()))
    }

    /// Resolves usage of a compound identifier.
    ///
    /// Note that currently only compound identifier of length 2 are supported
    /// and resolution will fail if the identifier has more than two parts.
    pub fn resolve_compound_ident(
        &self,
        idents: &[Ident],
    ) -> Result<Rc<SourceAnnotation>, ResolutionError> {
        // TODO: deal with multiple schemas (idents.len() > 2). Currently
        // defaulting implicitly to the public schema within a database.
        // TODO: change type from Vec (which is unbounded) to an enum with a
        // fixed number of variants
        // TODO: take into account the `search_path` (PG-specific, but suggests
        // the resolution logic should be behind a trait which is implemented
        // per database variant)
        if idents.len() != 2 {
            return Err(ResolutionError::InvariantFailed(
                InvariantFailedError::MaxCompoundIdentLengthExceeded(idents.len() as u8),
            ));
        }
        let first_part = UniCase::new(idents[0].to_string());
        let second_part = UniCase::new(idents[1].to_string());

        self.bindings
            .iter()
            .find_map(|relation| {
                if relation.name == first_part {
                    Some(relation.projection.clone())
                } else {
                    None
                }
            })
            .and_then(|projection| {
                projection
                    .columns
                    .iter()
                    .find_map(|(source_annotation, column)| {
                        if Some(true) == column.as_ref().map(|c| c == &second_part) {
                            Some(source_annotation.clone())
                        } else {
                            None
                        }
                    })
            })
            .or_else(|| match &self.parent {
                Some(parent) => parent.resolve_compound_ident(idents).ok(),
                None => None,
            })
            .ok_or(ResolutionError::NoSuchCompoundIdentifier(format!(
                "{}.{}",
                idents[0], idents[1]
            )))
            .inspect_err(|_| {
                #[cfg(test)]
                self.dump()
            })
    }

    /// Add a table/view/subquery to the current scope.
    pub fn add_relation(
        &mut self,
        relation: NamedRelation,
    ) -> Result<NamedRelation, ResolutionError> {
        self.bindings.push(relation.clone());

        #[cfg(test)]
        self.dump();

        Ok(relation.clone())
    }

    pub fn resolve_relation(
        &self,
        name: &UniCase<String>,
    ) -> Result<NamedRelation, ResolutionError> {
        self.bindings
            .iter()
            .find(|binding| &binding.name == name)
            .map(|relation| relation.clone())
            .ok_or(ResolutionError::NoSuchRelation(name.to_string()))
    }

    #[cfg(test)]
    fn dump(&self) {
        eprintln!(
            "SCOPE {}: {}",
            self.depth,
            test_utils::OneLine(&self.bindings)
        );
        if let Some(parent) = &self.parent {
            parent.dump();
        }
    }
}

#[cfg(test)]
mod test_utils {
    use std::{
        fmt::{self, Display, Formatter},
        ops::Deref,
    };

    use crate::source_annotation::{NamedRelation, SourceAnnotationItem, TableColumn};

    pub(super) struct OneLine<'a>(pub &'a Vec<NamedRelation>);

    impl<'a> Display for OneLine<'a> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            f.write_str(
                &self
                    .0
                    .iter()
                    .map(|nr| {
                        format!(
                            "{}:[{}]",
                            &nr.name,
                            nr.projection
                                .columns
                                .iter()
                                .map(|(source, name)| {
                                    let source_items = source
                                        .items
                                        .iter()
                                        .map(|item| match item.deref() {
                                            SourceAnnotationItem::TableColumn(TableColumn {
                                                table,
                                                column,
                                            }) => {
                                                format!("C({}.{})", table, column)
                                            }
                                            SourceAnnotationItem::Value(v) => {
                                                format!("Value({})", v)
                                            }
                                            SourceAnnotationItem::ColumnOfValues => {
                                                format!("ColOfValues")
                                            }
                                            SourceAnnotationItem::TypedString(
                                                data_type,
                                                string,
                                            ) => {
                                                format!("TypedString({},{})", data_type, string)
                                            }
                                            SourceAnnotationItem::IntroducedString(
                                                string,
                                                value,
                                            ) => {
                                                format!("IntroducedString({},{})", string, value)
                                            }
                                            SourceAnnotationItem::FunctionCall(name) => {
                                                format!("FunctionCall({})", name)
                                            }
                                        })
                                        .collect::<Vec<_>>()
                                        .join(" + ");
                                    format!(
                                        "({}){}",
                                        source_items,
                                        name.as_ref()
                                            .map(|name| format!("@{}", name))
                                            .unwrap_or(String::from(""))
                                    )
                                })
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(", "),
            )
        }
    }
}
