//! Types for representing and maintaining a stack of lexical scopes during AST
//! traversal.
use crate::{
    model::{
        InvariantFailedError, NamedRelation, Projection, ProjectionColumn, ResolutionError, Source,
        SqlIdent,
    },
    ProjectionColumnsIterator,
};
use core::ops::{Deref, DerefMut};
use std::{
    mem,
    rc::Rc,
    slice::{self},
};

/// A stack of [`Scope`] structs.
///
/// Implements `Deref<Target = Scope>` where `Scope` is the top of the stack
/// (current lexical scope).
#[derive(Debug, PartialEq, Eq, Default)]
pub struct ScopeStack {
    top: Scope,
}

impl ScopeStack {
    /// Push a new empty [`Scope`] onto the stack.
    pub(crate) fn push(&mut self) {
        let top = mem::take(&mut self.top);
        self.top = Scope {
            items: Vec::default(),
            parent: Some(Box::new(top)),
        }
    }

    /// Pop the top [`Scope`] of the stack.
    ///
    /// Panics if the top of the stack is the `root` [`Scope`].
    pub(crate) fn pop(&mut self) {
        let mut me = mem::take(self);
        match me.top {
            Scope {
                items: _,
                parent: None,
                ..
            } => panic!("Cannot pop the root scope"),
            Scope {
                items: _,
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
    /// The items in scope.
    ///
    /// Searching for items in scope is a linear search through this `Vec`. This will
    /// perform better than `HashMap` for a small number of items and is probably good
    /// enough.
    items: Vec<Rc<NamedRelation>>,

    /// The parent scope frame.
    parent: Option<Box<Scope>>,
}

impl Scope {
    /// Expand usage of a wildcard into the actual concrete table-columns it stands for.
    // TODO: this fn is building the Projection on demand but returning it in an
    // Rc which is odd, but just for consistency.
    // However it does seem to hint that Projection should be an enum with a
    // variant for representing concatenated sub-projections.
    pub fn resolve_wildcard(&self) -> Result<Rc<Projection>, ResolutionError> {
        if self.items.is_empty() {
            match &self.parent {
                Some(parent) => parent.resolve_wildcard(),
                None => Err(ResolutionError::InvariantFailed(
                    InvariantFailedError::EmptyScope,
                )),
            }
        } else {
            let projections: Vec<Rc<Projection>> = self
                .items
                .iter()
                .map(|relation| relation.projection.clone())
                .collect();

            Ok(Rc::new(Projection::Concatenated(projections)))
        }
    }

    /// Expand usage of a qualified wildcard into the actual concrete table-columns it stands for.
    pub fn resolve_qualified_wildcard(
        &self,
        idents: &[SqlIdent],
    ) -> Result<Rc<Projection>, ResolutionError> {
        if idents.len() > 1 {
            return Err(ResolutionError::UnsupportedCompoundIdentifierLength(
                idents.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
            ));
        }

        match SqlIdent::try_find_unique(&idents[0], &mut self.items.iter().cloned()) {
            Ok(Some(relation)) => Ok(relation.projection.clone()),
            Ok(None) => match &self.parent {
                Some(parent) => parent.resolve_qualified_wildcard(idents),
                None => Err(ResolutionError::NoSuchRelation(idents[0].to_string())),
            },
            Err(err) => Err(err.into()),
        }
    }

    /// Uniquely resolves an identifier against all relations that are in scope.
    pub fn resolve_ident(&self, ident: &SqlIdent) -> Result<Rc<Source>, ResolutionError> {
        let mut bindings_iter = AllColumnsIterator::new(self.items.iter());

        match SqlIdent::try_find_unique(ident, &mut bindings_iter) {
            Ok(Some(projection_column)) => Ok(projection_column.source.clone()),
            Ok(None) => match &self.parent {
                Some(parent) => parent.resolve_ident(ident),
                None => Err(ResolutionError::NoSuchIdentifier(ident.to_string())),
            },
            Err(err) => Err(err.into()),
        }
    }

    /// Resolves usage of a compound identifier.
    ///
    /// Note that currently only compound identifier of length 2 are supported
    /// and resolution will fail if the identifier has more than two parts.
    pub fn resolve_compound_ident(
        &self,
        idents: &[SqlIdent],
    ) -> Result<Rc<Source>, ResolutionError> {
        if idents.len() != 2 {
            return Err(ResolutionError::InvariantFailed(
                InvariantFailedError::MaxCompoundIdentLengthExceeded(idents.len() as u8),
            ));
        }
        match SqlIdent::try_find_unique(&idents[0], &mut self.items.iter().cloned()) {
            Ok(Some(named_relation)) => {
                match SqlIdent::find_unique(
                    &idents[1],
                    &mut named_relation.projection.columns_iter(),
                ) {
                    Ok(projection_column) => Ok(projection_column.source.clone()),
                    Err(err) => Err(err.into()),
                }
            }
            Ok(None) => match &self.parent {
                Some(parent) => parent.resolve_compound_ident(idents),
                None => Err(ResolutionError::NoSuchCompoundIdentifier(format!(
                    "{}.{}",
                    idents[0], idents[1]
                ))),
            },
            Err(err) => Err(err.into()),
        }
    }

    /// Add a table/view/subquery to the current scope.
    pub fn add_relation(
        &mut self,
        relation: Rc<NamedRelation>,
    ) -> Result<Rc<NamedRelation>, ResolutionError> {
        self.items.push(relation.clone());
        Ok(relation)
    }

    pub fn resolve_relation(&self, ident: &SqlIdent) -> Result<Rc<NamedRelation>, ResolutionError> {
        match SqlIdent::find_unique(ident, &mut self.items.iter().cloned()) {
            Ok(found) => Ok(found.clone()),
            Err(err) => match &self.parent {
                Some(parent) => parent.resolve_relation(ident),
                None => Err(ResolutionError::from(err)),
            },
        }
    }
}

struct AllColumnsIterator<'a> {
    relations: slice::Iter<'a, Rc<NamedRelation>>,
    columns: Option<ProjectionColumnsIterator>,
    done: bool,
}

impl<'a> AllColumnsIterator<'a> {
    fn new(relations: slice::Iter<'a, Rc<NamedRelation>>) -> Self {
        Self {
            relations,
            columns: None,
            done: false,
        }
    }
}

impl<'a> Iterator for AllColumnsIterator<'a> {
    type Item = Rc<ProjectionColumn>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.done {
            match self.columns.as_mut().and_then(|iter| iter.next()) {
                Some(projection_column) => Some(projection_column.clone()),
                None => {
                    self.columns = self
                        .relations
                        .next()
                        .map(|named_relation| named_relation.projection.columns_iter());
                    if self.columns.is_some() {
                        self.next()
                    } else {
                        self.done = true;
                        None
                    }
                }
            }
        } else {
            None
        }
    }
}
#[cfg(test)]
mod test {
    use crate::{
        model::{CanonicalIdent, Column, SqlIdent, Table},
        model::{SourceItem, TableColumn},
    };

    use super::*;
    use test_case::test_case;

    fn stack(frames: &[&[NamedRelation]]) -> ScopeStack {
        let mut stack = ScopeStack::default();

        for frame in frames {
            for relation in *frame {
                stack.top.add_relation(Rc::new(relation.clone())).unwrap();
            }
            stack.push()
        }

        stack
    }

    fn assert_ok(result: Result<Rc<NamedRelation>, ResolutionError>) {
        match result {
            Ok(_) => (),
            Err(err) => panic!("ok_ok failed! {:#?}", err),
        }
    }

    fn relation(name: &str, columns: &[&str]) -> NamedRelation {
        let name = Rc::new(CanonicalIdent::from(name));
        let columns = columns
            .iter()
            .map(|c| Rc::new(CanonicalIdent::from(*c)))
            .map(|c| Rc::new(Column { name: c }))
            .collect::<Vec<_>>();
        let table = Rc::new(Table {
            name: Rc::clone(&name),
            columns,
            primary_key: Vec::default(),
        });
        NamedRelation {
            name: Rc::new(SqlIdent::Canonical(name.deref().clone())),
            projection: Projection::Columns(Vec::from_iter(table.columns.iter().map(|column| {
                ProjectionColumn::new(
                    Rc::new(Source::single(SourceItem::TableColumn(TableColumn {
                        table: Rc::clone(&table),
                        column: Rc::clone(&column),
                    }))),
                    Some(Rc::new(SqlIdent::Canonical(
                        column.name.deref().clone().into(),
                    ))),
                )
                .into()
            })))
            .into(),
        }
    }

    #[test_case(stack(&[&[relation("users", &["id"])]]), SqlIdent::unquoted("users"), &assert_ok)]
    fn resolve_relation<F>(stack: ScopeStack, ident: SqlIdent, assert_pass: F)
    where
        F: Fn(Result<Rc<NamedRelation>, ResolutionError>) -> (),
    {
        assert_pass(stack.resolve_relation(&ident))
    }
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
