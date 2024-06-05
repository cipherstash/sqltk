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
use crate::ColumnWithOptionalAlias;

/// `ExprSource` records the provenance of a single `Expr` node.
///
/// `ExprSource` refers to one or more [`ExprSource`]s.
///
/// For example in SQL statement below, `full_name` is an expression with two
/// `TableColumn` sources: `first_name` and `last_name`
///
/// ```sql
/// SELECT (users.first_name || ' ' || users.last_name) as full_name from user;
/// ```
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum ExprSource {
    /// Refers to a column of a specific table
    TableColumn(TableColumn),

    /// Refers to an entire projection of a sub-query.
    /// Required because an Expr node can contain a Query node.
    Projection(Rc<Projection>),

    /// A literal or a placeholder
    // TODO: split out values and placeholders
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
        arg_sources: Vec<Rc<ExprSource>>,
    },

    Multiple(Vec<Rc<ExprSource>>),

    ResolvedWildcard(Vec<Rc<ColumnWithOptionalAlias>>),
}

impl ExprSource {
    /// Recursively resolves an `ExprSource` into a `Vec<TableColumn>`.
    ///
    /// The leaf nodes of the tree of `ExprSource`s do not necessarily resolve
    /// into `TableColumn`s and the length of the returned `Vec` is unrelated
    /// to the number of leaf nodes and may also be empty.
    pub fn resolve_to_table_columns(&self) -> Vec<TableColumn> {
        let mut output: Vec<TableColumn> = Vec::new();
        self.resolve_to_table_columns_internal(&mut output);
        output
    }

    fn resolve_to_table_columns_internal(&self, output: &mut Vec<TableColumn>) {
        match self {
            ExprSource::TableColumn(tc) => output.push(tc.clone()),
            ExprSource::Projection(projection) => {
                for col in projection.columns.deref() {
                    col.source.resolve_to_table_columns_internal(output)
                }
            }
            ExprSource::FunctionCall { arg_sources, .. } => {
                for source in arg_sources {
                    source.resolve_to_table_columns_internal(output)
                }
            }
            ExprSource::Multiple(sources) => {
                for source in sources {
                    source.resolve_to_table_columns_internal(output)
                }
            }
            ExprSource::ResolvedWildcard(columns) => {
                for col in columns {
                    col.source.resolve_to_table_columns_internal(output)
                }
            }
            _ => {}
        }
    }
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
