use std::{
    marker::PhantomData,
    ops::{ControlFlow, Deref},
    rc::Rc,
};

use sqlparser::ast::{Insert, Query, SelectItem, Statement};
use sqltk::{visitor_extensions::VisitorExtensions, Break, Visitable, Visitor};

use crate::{
    Annotate, AnnotateMut, CanonicalIdent, ColumnWritten, InsertProvenance, Projection, Provenance,
    ResolutionError, SchemaOps, SqlIdent, Table,
};

#[derive(Debug)]
pub struct BuildInsertProvenance<'ast, State>(PhantomData<&'ast ()>, PhantomData<State>);

impl<'ast, State> Default for BuildInsertProvenance<'ast, State>
where
    State: SchemaOps
        + Annotate<'ast, Query, Projection>
        + Annotate<'ast, Vec<SelectItem>, Projection>
        + AnnotateMut<'ast, Statement, Provenance>,
{
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<'ast, State> Visitor<'ast> for BuildInsertProvenance<'ast, State>
where
    State: SchemaOps
        + Annotate<'ast, Query, Projection>
        + Annotate<'ast, Vec<SelectItem>, Projection>
        + AnnotateMut<'ast, Statement, Provenance>,
{
    type Error = ResolutionError;
    type State = State;

    fn exit<N: Visitable<'ast>>(
        &self,
        node: &'ast N,
        mut state: State,
    ) -> ControlFlow<Break<Self::State, Self::Error>, Self::State> {
        if let Some(statement) = node.downcast_ref::<Statement>() {
            match statement {
                Statement::Insert(Insert {
                    table_name,
                    columns,
                    source,
                    returning,
                    ..
                }) => {
                    let into_table: Result<Rc<Table>, _> = state
                        .get_schema()
                        .resolve_table(&SqlIdent::from(table_name.0.last().unwrap()))
                        .map_err(ResolutionError::from);

                    let source: Result<Option<Rc<Projection>>, _> = source
                        .as_ref()
                        .map(|source| state.get_annotation(source.deref()))
                        .transpose()
                        .map_err(ResolutionError::from);

                    let columns_written: Result<Vec<ColumnWritten>, _> = source
                        .map(|maybe_source| {
                            maybe_source
                                .map(|projection| {
                                    columns
                                        .iter()
                                        .map(|ident| {
                                            Rc::new(CanonicalIdent::from(ident.value.as_str()))
                                        })
                                        .zip(projection.columns.iter().cloned())
                                        .map(ColumnWritten::from)
                                        .collect::<Vec<_>>()
                                })
                                .unwrap_or_default()
                        })
                        .map_err(ResolutionError::from);

                    let returning: Result<Option<Rc<Projection>>, _> = returning
                        .as_ref()
                        .map(|items| state.get_annotation(items))
                        .transpose()
                        .map_err(ResolutionError::from);

                    let result = into_table
                        .and_then(|into_table| {
                            columns_written.and_then(|columns_written| {
                                returning.map(|returning| (into_table, columns_written, returning))
                            })
                        })
                        .map(|(into_table, columns_written, returning)| {
                            state.set_annotation(
                                statement,
                                Provenance::Insert(
                                    InsertProvenance {
                                        into_table,
                                        columns_written,
                                        returning,
                                    }
                                    .into(),
                                ),
                            );
                        });

                    match result {
                        Ok(_) => self.continue_with_state(state),
                        Err(err) => self.break_with_error(err),
                    }
                }
                _ => self.continue_with_state(state),
            }
        } else {
            self.continue_with_state(state)
        }
    }
}
