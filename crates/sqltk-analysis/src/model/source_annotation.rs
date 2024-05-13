//! Types that describe the provenance of individual [`sqlparser::ast::Expr`]
//! and [`sqlparser::ast::SelectItem`] nodes.

use core::fmt::Display;
use std::fmt;

use derive_more::IntoIterator;
use derive_new::new;
use sqlparser::ast::{DataType, Value};
use std::collections::BTreeSet;
use std::rc::Rc;

use crate::model::Projection;
use crate::model::{Column, Named, SqlIdent, Table};

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
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, IntoIterator)]
pub struct Source {
    /// A set of one or more items.
    #[into_iterator(owned, ref)]
    pub items: BTreeSet<Rc<SourceItem>>,
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

    pub fn merge(a: &Rc<Self>, b: &Rc<Self>) -> Rc<Self> {
        Rc::new(Self {
            items: BTreeSet::from_iter(a.items.iter().chain(b.items.iter()).cloned()),
        })
    }
}

/// A `SourceItem` describes the origin of a term used an expression.
///
/// For example:
/// -  an identifier `Expr` could be refering to a table column
/// -  a value `Expr` could be referring to a literal or a placeholder parameter
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum SourceItem {
    /// Refers to a Table + Column
    TableColumn(TableColumn),

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
}

impl From<SourceItem> for Rc<Source> {
    fn from(value: SourceItem) -> Self {
        Rc::new(Source::single(value))
    }
}

/// Describes a reference to a table + column.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd, new)]
pub struct TableColumn {
    pub table: Rc<Table>,
    pub column: Rc<Column>,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct NamedRelation {
    pub name: Rc<SqlIdent>,
    pub projection: Rc<Projection>,
}

impl Display for TableColumn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}.{}", self.table, self.column))
    }
}
