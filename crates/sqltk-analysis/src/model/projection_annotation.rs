use std::rc::Rc;

use unicase::UniCase;

use crate::{
    schema::Table,
    source_annotation::{SourceAnnotation, SourceAnnotationItem, TableColumn},
};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum ProjectionAnnotation {
    Query(Projection),

    /// An insert has a projection that can be referenced by other sub-queries
    /// (RETURNING). It also has an optional ColumnsWritten.
    Insert {
        table: String,
        columns_written: InsertColumnsWritten,
        returning: Option<Projection>,
    },
}

impl ProjectionAnnotation {
    pub fn projection(&self) -> Option<&Projection> {
        match self {
            Self::Query(projection) => Some(projection),
            Self::Insert { ref returning, .. } => returning.as_ref(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Projection {
    pub columns: Vec<(Rc<SourceAnnotation>, Option<UniCase<String>>)>,
}

impl From<&Table> for Projection {
    fn from(table: &Table) -> Self {
        Projection {
            columns: table
                .columns
                .iter()
                .map(|c| {
                    (
                        Rc::new(SourceAnnotation::single(SourceAnnotationItem::TableColumn(
                            TableColumn::new(table.name.clone(), c.clone()),
                        ))),
                        Some(c.clone()),
                    )
                })
                .collect(),
        }
    }
}

impl FromIterator<(Rc<SourceAnnotation>, Option<UniCase<String>>)> for Projection {
    fn from_iter<I: IntoIterator<Item = (Rc<SourceAnnotation>, Option<UniCase<String>>)>>(
        iter: I,
    ) -> Self {
        let mut columns = Vec::new();
        for i in iter {
            columns.push(i.clone());
        }

        Projection { columns }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum InsertColumnsWritten {
    Named(Vec<(String, Rc<SourceAnnotation>)>),
    Defaults,
}
