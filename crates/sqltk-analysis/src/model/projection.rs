use std::{ops::Deref, rc::Rc};

use crate::{
    model::schema::{SqlIdent, Table},
    model::source_item::{SourceItem, TableColumn},
};

use derive_more::AsRef;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Projection
where
    Self: 'static,
{
    /// A Projection of columns from a single table/view/SELECT
    Columns(Vec<Rc<ProjectionColumn>>),

    /// A Projection composed from two other Projections (recursively).
    /// A wildard column selection could produce a projection of this variant
    /// when more than one relation is in scope.
    Concatenated(Vec<Rc<Projection>>),
}

impl Projection {
    pub fn columns_iter(self: &Rc<Self>) -> ProjectionColumnsIterator {
        ProjectionColumnsIterator::new(self.clone())
    }
}

pub struct ProjectionColumnsIterator
where
    Self: 'static,
{
    captured: Vec<Rc<ProjectionColumn>>,
}

impl ProjectionColumnsIterator {
    fn new(projection: Rc<Projection>) -> Self {
        // NOTE: figure out how to implement this iterator *without* allocating
        // a Vec to capture the columns that we need to iterate over.
        let mut captured = match projection.deref() {
            Projection::Columns(columns) => Vec::from_iter(columns.iter().cloned()),
            Projection::Concatenated(projections) => Vec::from_iter(
                projections
                    .iter()
                    .cloned()
                    .flat_map(|projection| projection.columns_iter()),
            ),
        };

        // The Iterator implementation works by pop'ing the last element of the
        // Vec (which is more efficient than taking an element from the
        // beginning of the Vec).  So the Vec needs to be in reverse order.
        captured.reverse();

        Self { captured }
    }
}

impl Iterator for ProjectionColumnsIterator {
    type Item = Rc<ProjectionColumn>;

    fn next(&mut self) -> Option<Self::Item> {
        self.captured.pop()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, AsRef)]
pub struct ProjectionColumn {
    #[as_ref(forward)]
    pub source: Rc<SourceItem>,
    pub alias: Option<Rc<SqlIdent>>,
}

impl ProjectionColumn {
    pub fn new(source: Rc<SourceItem>, alias: Option<Rc<SqlIdent>>) -> Self {
        Self { source, alias }
    }
}

impl From<&Rc<Table>> for Projection {
    fn from(table: &Rc<Table>) -> Self {
        Projection::Columns(
            table
                .deref()
                .columns
                .iter()
                .map(|column| ProjectionColumn {
                    source: SourceItem::TableColumn(TableColumn::new(
                        Rc::clone(table),
                        Rc::clone(column),
                    ))
                    .into(),
                    alias: Some(Rc::new(SqlIdent::Canonical(column.name.deref().clone()))),
                })
                .map(Rc::new)
                .collect(),
        )
    }
}
