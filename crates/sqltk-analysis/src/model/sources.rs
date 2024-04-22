//! Types that describe the provenance of individual [`sqlparser::ast::Expr`]
//! and [`sqlparser::ast::SelectItem`] nodes.

use core::fmt::Display;
use std::fmt;

use derive_new::new;
use sqlparser::ast::{DataType, Query, Value};
use std::collections::BTreeSet;
use std::rc::Rc;

use super::schema::Table;

/// A `Source` records the provenance of a single `Expr` or `SelectItem` node.
///
/// A `Source` refers to one or more [`SourceItem`]s.
///
/// For example in SQL statement below, `full_name` is an expression with two
/// `TableColumn` sources: `first_name` and `last_name`
///
/// ```sql
/// SELECT (users.first_name || ' ' || users.last_name) as full_name from user;
/// ```
///
/// `Source` is not implemented recursively. When deriving the source for an
/// expression that contains subexpression with existing `Source`s, the items
/// from the subexpression are merged into the parent.
///
/// Internally the items are stored in a [`BTreeSet<Rc<SourceItem>>`]. Only the
/// [`Rc`] is cloned when merging, not the `SourceItem`s themselves. This means
/// arbitrary large AST values contained within a `SourceItem` are allocated
/// only once.
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Source {
    /// A set of one or more items.
    items: BTreeSet<Rc<SourceItem>>,
}

impl Source {
    /// Create a `Source` containing a single `SourceItem`.
    pub fn single(item: impl Into<Rc<SourceItem>>) -> Source {
        Self {
            items: BTreeSet::from([item.into()]),
        }
    }

    /// Create a `Source` containing a multiple `SourceItem`s.
    pub fn multiple(
        item: impl Into<Rc<SourceItem>>,
        rest: impl IntoIterator<Item = Rc<SourceItem>>,
    ) -> Source {
        let mut items = BTreeSet::<Rc<SourceItem>>::new();
        items.insert(item.into());
        items.extend(rest);
        Self { items }
    }

    /// Merge two `Sources` into a new `Source`.
    ///
    /// This is used to compose sources for an [`sqlparser::ast::Expr`] that
    /// contains sub-expressions.
    pub fn merge(a: &Self, b: &Self) -> Self {
        Self {
            items: BTreeSet::from_iter(a.items.iter().chain(b.items.iter()).cloned()),
        }
    }
}

/// A `SourceItem` describes the origin of a term used an expression.
///
/// For example:
/// -  an identifier `Expr` could be refering to a table column
/// -  a value `Expr` could be referring to a literal or a placeholder parameter
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum SourceItem {
    /// Refers to a Table + Column
    TableColumn(TableColumn),

    /// A literal or a placeholder
    Value(Value),

    /// Refers to a "typed string". This is fairly obscure and is included for completeness.
    /// Annoyingly a "typed string" is not a value expression in sqlparser.
    TypedString(DataType, String),

    /// Refers to a "introduced string". This is fairly obscure and is included for completeness.
    /// Annoyingly a "introduced string" is not a value expression in sqlparser.
    IntroducedString(String, Value),

    /// A value produced by a function call.
    FunctionCall(String),
}

impl From<SourceItem> for Source {
    fn from(value: SourceItem) -> Self {
        Source::single(value)
    }
}

/// Describes a reference to a table + column.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd, new)]
pub struct TableColumn {
    pub table: String,
    pub column: ColumnRef,
}

/// In `sqltk`, a `Relation` is a component of a SQL statement that has rows &
/// columns and can be referred to in joins etc. Tables, views and subqueries
/// are all relations.
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Relation {
    /// A table or view with an optional alias
    TableLike(Table, Option<String>),

    /// A subquery with an optional alias
    SubQuery(Query, Vec<(ColumnRef, Source)>, Option<String>),
}

/// A reference to a column either by identifier or ordinal position.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum ColumnRef {
    /// For a column referenced by name
    Identifier(String),

    /// For a column referenced by ordinal position
    Ordinal(usize),
}

impl Display for ColumnRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColumnRef::Identifier(ident) => f.write_str(ident),
            ColumnRef::Ordinal(ordinal) => f.write_str(&ordinal.to_string()),
        }
    }
}

impl Display for TableColumn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}.{}", self.table, self.column))
    }
}
