use std::{ops::Deref, rc::Rc};

use crate::{
    model::schema::{Named, SqlIdent, Table},
    model::source_annotation::{Source, SourceItem, TableColumn},
};

use derive_more::{AsRef, IntoIterator};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, IntoIterator)]
pub struct Projection {
    #[into_iterator(owned, ref)]
    pub columns: Vec<Rc<ProjectionColumn>>,
}

// #[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
// pub enum Projection {
//     /// A Projection of columns from a single table/view/SELECT
//     Columns(Vec<Rc<ProjectionColumn>>),

//     /// A Projection composed from two other Projections (recursively).
//     /// A wildard column selection could produce a projection of this variant
//     /// when more than one relation is in scope.
//     Concatenated(Rc<Projection>, Rc<Projection>)
// }

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, AsRef)]
pub struct ProjectionColumn {
    #[as_ref(forward)]
    pub source: Rc<Source>,
    pub alias: Option<Rc<SqlIdent>>,
}

impl ProjectionColumn {
    pub fn new(source: Rc<Source>, alias: Option<Rc<SqlIdent>>) -> Self {
        Self { source, alias }
    }
}

impl Named for ProjectionColumn {
    type Name = Option<Rc<SqlIdent>>;

    fn name(&self) -> &Self::Name {
        &self.alias
    }
}

impl From<&Rc<Table>> for Projection {
    fn from(table: &Rc<Table>) -> Self {
        Projection {
            columns: table
                .deref()
                .columns
                .iter()
                .map(|column| ProjectionColumn {
                    source: Source::single(SourceItem::TableColumn(TableColumn::new(
                        Rc::clone(table),
                        Rc::clone(column),
                    )))
                    .into(),
                    alias: Some(Rc::new(SqlIdent::Canonical(column.name.deref().clone()))),
                })
                .map(Rc::new)
                .collect(),
        }
    }
}

impl FromIterator<Rc<ProjectionColumn>> for Projection {
    fn from_iter<I: IntoIterator<Item = Rc<ProjectionColumn>>>(iter: I) -> Self {
        let mut columns = Vec::new();
        for i in iter {
            columns.push(i.clone());
        }

        Projection { columns }
    }
}
