//! Traits and types used used to annotate [`Expr`] nodes with their sources.

use crate::model::Projection;
use crate::model::ResolutionError;
use crate::model::{CanonicalIdent, SqlIdent};
use crate::model::{ColumnWritten, InsertProvenance, Provenance, SelectProvenance};
use crate::ProvenanceStateBounds;

use sqltk::Visitor;
use sqltk::{flow, VisitorControlFlow, VisitorStack};

use sqlparser::ast::{Insert, Statement};

use std::any::Any;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Default)]
pub struct AnnotateProvenance<'ast, State> {
    stack: VisitorStack<'ast, State, ResolutionError>,
}

impl<'ast, State> Visitor<'ast, State, ResolutionError> for AnnotateProvenance<'ast, State>
where
    State: 'ast + ProvenanceStateBounds<'ast>,
{
    fn exit<N: 'static>(
        &self,
        node: &'ast N,
        mut state: State,
    ) -> VisitorControlFlow<'ast, State, ResolutionError> {
        if let Some(statement) = (node as &dyn Any).downcast_ref::<Statement>() {
            match statement {
                Statement::Query(query) => match state.expect_annotation(query.deref()) {
                    Ok(projection) => {
                        state.add_annotation(
                            statement,
                            Provenance::Select(
                                SelectProvenance {
                                    projection: projection.clone(),
                                }
                                .into(),
                            ),
                        );
                        flow::cont(state)
                    }
                    Err(err) => flow::error(err.into(), state),
                },
                Statement::Insert(insert) => annotate_insert(insert, state, statement),
                // Statement::Update { table, assignments, from, selection, returning } => Annotate
                // Statement::Delete(delete) => annotate_delete
                _ => flow::cont(state),
            }
        } else {
            flow::cont(state)
        }
    }
}

fn annotate_insert<'ast, State>(
    insert: &'ast Insert,
    mut state: State,
    statement: &'ast Statement,
) -> VisitorControlFlow<'ast, State, ResolutionError>
where
    State: 'ast + ProvenanceStateBounds<'ast>,
{
    let Insert {
        table_name,
        columns,
        source,
        returning,
        ..
    } = insert;
    let table: Result<_, ResolutionError> = state
        .get_schema()
        .resolve_table(&SqlIdent::from(table_name.0.last().unwrap()))
        .map_err(|err| err.into());

    let source: Result<Option<Projection>, ResolutionError> = match source {
        Some(source) => state
            .expect_annotation(source.deref())
            .map(|projection| projection.deref().clone())
            .map(Some)
            .map_err(ResolutionError::from),
        None => Ok(None),
    };

    let columns_written: Result<Vec<ColumnWritten>, ResolutionError> = match source {
        Ok(Some(projection)) => columns
            .clone()
            .into_iter()
            .map(|ident| CanonicalIdent::from(ident.value))
            .zip(projection.columns.into_iter())
            .map(|(ident, column)| {
                Ok(ColumnWritten {
                    column: ident.into(),
                    data: column.source.clone(),
                })
            })
            .collect::<Result<Vec<_>, _>>(),
        Ok(None) => Ok(vec![]),
        Err(err) => Err(err.into()),
    };

    let returning: Result<Option<Rc<Projection>>, ResolutionError> = match returning {
        Some(select_items) => {
            match select_items
                .iter()
                .map(|item| state.expect_annotation(item))
                .collect::<Result<Vec<_>, _>>()
            {
                Ok(projection_columns) => {
                    if projection_columns.len() > 0 {
                        Ok(Some(
                            Projection {
                                columns: projection_columns
                                    .iter()
                                    .map(|v| v.deref())
                                    .flatten()
                                    .map(|source| source.clone())
                                    .collect(),
                            }
                            .into(),
                        ))
                    } else {
                        Ok(None)
                    }
                }
                Err(err) => Err(err.into()),
            }
        }
        None => Ok(None),
    };

    match (table, columns_written, returning) {
        (Ok(table), Ok(columns_written), Ok(returning)) => {
            state.add_annotation(
                statement,
                Provenance::Insert(
                    InsertProvenance {
                        into_table: table,
                        columns_written,
                        returning,
                    }
                    .into(),
                ),
            );
            flow::cont(state)
        }

        (table, columns_written, returning) => match table.and(columns_written).and(returning) {
            Ok(_) => unreachable!("We know that at least one of the Result values is an error"),
            Err(err) => flow::error(err.into(), state),
        },
    }
}
