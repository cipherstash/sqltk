use std::{ops::Deref, rc::Rc};

use crate::{
    model::{
        expr_source::{ExprSource, TableColumn},
        schema::{SqlIdent, Table},
    },
    ColumnWithOptionalAlias,
};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Projection
where
    Self: 'static,
{
    pub columns: Vec<Rc<ColumnWithOptionalAlias>>,
}

impl From<&Rc<Table>> for Projection {
    fn from(table: &Rc<Table>) -> Self {
        Projection {
            columns: table
                .deref()
                .columns
                .iter()
                .map(|column| ColumnWithOptionalAlias {
                    source: ExprSource::TableColumn(TableColumn::new(
                        Rc::clone(table),
                        Rc::clone(column),
                    ))
                    .into(),
                    alias: Some(Rc::new(SqlIdent::Canonical(column.name.deref().clone()))),
                })
                .map(Rc::new)
                .collect(),
        }
    }
}
