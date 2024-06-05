use std::rc::Rc;

use crate::{
    model::{
        expr_source::ExprSource,
        projection::Projection,
        schema::{CanonicalIdent, Table},
    },
    ColumnWithOptionalAlias,
};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Provenance {
    Select(Rc<SelectProvenance>),

    Insert(Rc<InsertProvenance>),

    Update(Rc<UpdateProvenance>),

    Delete(Rc<DeleteProvenance>),
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct SelectProvenance {
    pub projection: Rc<Projection>, // TODO: capture columns written & columns deleted
                                    // because SELECTs that use CTEs can contain embedded
                                    // INSERT/DELETE/UPDATE sub-statements
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct InsertProvenance {
    pub into_table: Rc<Table>,
    pub returning: Option<Rc<Projection>>,
    pub columns_written: Vec<ColumnWritten>,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct UpdateProvenance {
    pub update_table: Rc<Table>,
    pub returning: Option<Rc<Projection>>,
    pub columns_written: Vec<ColumnWritten>,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct DeleteProvenance {
    pub from_table: Rc<Table>,
    pub returning: Option<Rc<Projection>>,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct ColumnWritten {
    pub column: Rc<CanonicalIdent>,
    pub data: Rc<ExprSource>,
}

impl From<(Rc<CanonicalIdent>, Rc<ColumnWithOptionalAlias>)> for ColumnWritten {
    fn from(value: (Rc<CanonicalIdent>, Rc<ColumnWithOptionalAlias>)) -> Self {
        Self {
            column: value.0,
            data: value.1.source.clone(),
        }
    }
}
