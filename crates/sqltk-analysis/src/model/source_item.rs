//! Types that describe the provenance of individual [`sqlparser::ast::Expr`]
//! and [`sqlparser::ast::SelectItem`] nodes.

use core::fmt::Display;
use std::fmt;
use std::ops::Deref;

use derive_new::new;
use sqlparser::ast::{DataType, Value};
use std::rc::Rc;

use crate::model::Projection;
use crate::model::{Column, SqlIdent, Table};

/// A `SourceItem` records the provenance of a single `Expr` or `SelectItem` node.
///
/// A `SourceItem` refers to one or more [`SourceItem`]s.
///
/// For example in SQL statement below, `full_name` is an expression with two
/// `TableColumn` sources: `first_name` and `last_name`
///
/// ```sql
/// SELECT (users.first_name || ' ' || users.last_name) as full_name from user;
/// ```
///
/// `SourceItem` is not implemented recursively. When deriving the source for an
/// expression that contains subexpression with existing `SourceItem`s, the items
/// from the subexpression are merged into the parent.
///
/// Internally the items are stored in a [`BTreeSet<Rc<SourceItem>>`]. Only the
/// [`Rc`] is cloned when merging, not the `SourceItem`s themselves. This means
/// arbitrary large AST values contained within a `SourceItem` are allocated
/// only once.
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum SourceItem {
    /// Refers to a Table + Column
    TableColumn(TableColumn),

    /// Refers to an entire projection of a sub-query
    Projection(Rc<Projection>),

    /// A literal or a placeholder
    Value(Value),

    /// A column of a `VALUES` expression (which can contain multiple rows).
    /// The values themselves are not recorded. This decision may be revised.
    ColumnOfValues,

    /// Refers to a "typed string". This is fairly obscure and is included for
    /// completeness.  Annoyingly a "typed string" is not a value expression in
    /// sqlparser.
    TypedString(DataType, String),

    /// Refers to a "introduced string". This is fairly obscure and is included
    /// for completeness.  Annoyingly a "introduced string" is not a value
    /// expression in sqlparser.
    IntroducedString(String, Value),

    /// A value produced by a function call.
    FunctionCall {
        ident: SqlIdent,
        arg_sources: Vec<Rc<SourceItem>>,
    },

    Multiple(Vec<Rc<SourceItem>>),
}

/// Describes a reference to a table + column.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd, new)]
pub struct TableColumn {
    pub table: Rc<Table>,
    pub column: Rc<Column>,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, new)]
pub struct NamedRelation {
    pub name: Rc<SqlIdent>,
    pub projection: Rc<Projection>,
}

impl From<Rc<Table>> for NamedRelation {
    fn from(table: Rc<Table>) -> Self {
        Self {
            name: Rc::new(SqlIdent::Canonical(table.name.deref().clone())),
            projection: Rc::new(Projection::from(&table)),
        }
    }
}

impl Display for TableColumn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}.{}", self.table, self.column))
    }
}
