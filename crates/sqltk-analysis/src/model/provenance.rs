use std::rc::Rc;

use crate::{
    model::projection_annotation::Projection,
    model::schema::{CanonicalIdent, Table},
    model::source_annotation::Source,
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
    pub data: Rc<Source>,
}
