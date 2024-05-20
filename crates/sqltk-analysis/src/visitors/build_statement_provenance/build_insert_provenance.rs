use std::{marker::PhantomData, ops::Deref, rc::Rc};

use sqlparser::ast::{Insert, Query, SelectItem, Statement};
use sqltk::{
    prelude::{flow, VisitorControlFlow},
    Visitable, Visitor,
};

use crate::{
    Annotate, CanonicalIdent, ColumnWritten, InsertProvenance, Projection, Provenance,
    ResolutionError, SchemaOps, SqlIdent, Table,
};

#[derive(Debug)]
pub struct BuildInsertProvenance<'ast, State>(PhantomData<&'ast ()>, PhantomData<State>);

impl<'ast, State> Default for BuildInsertProvenance<'ast, State>
where
    State: SchemaOps
        + Annotate<'ast, Statement, Provenance>
        + Annotate<'ast, Query, Projection>
        + Annotate<'ast, Vec<SelectItem>, Projection>,
{
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<'ast, State> Visitor<'ast> for BuildInsertProvenance<'ast, State>
where
    State: SchemaOps
        + Annotate<'ast, Statement, Provenance>
        + Annotate<'ast, Query, Projection>
        + Annotate<'ast, Vec<SelectItem>, Projection>,
{
    type Error = ResolutionError;
    type State = State;

    fn exit<N: Visitable<'ast>>(
        &self,
        node: &'ast N,
        mut state: State,
    ) -> VisitorControlFlow<'ast, Self::State, Self::Error> {
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
                        .map_err(|err| ResolutionError::from(err));

                    let source: Result<Option<Rc<Projection>>, _> = source
                        .as_ref()
                        .map(|source| state.expect_annotation(source.deref()))
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
                                        .zip(projection.columns_iter())
                                        .map(ColumnWritten::from)
                                        .collect::<Vec<_>>()
                                })
                                .unwrap_or(vec![])
                        })
                        .map_err(ResolutionError::from);

                    let returning: Result<Option<Rc<Projection>>, _> = returning
                        .as_ref()
                        .map(|items| state.expect_annotation(items))
                        .transpose()
                        .map_err(ResolutionError::from);

                    let result = into_table
                        .and_then(|into_table| {
                            columns_written.and_then(|columns_written| {
                                returning.and_then(|returning| {
                                    Ok((into_table, columns_written, returning))
                                })
                            })
                        })
                        .and_then(|(into_table, columns_written, returning)| {
                            state.add_annotation(
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
                            Ok(())
                        });

                    match result {
                        Ok(_) => flow::cont(state),
                        Err(err) => flow::error(err.into()),
                    }
                }
                _ => flow::cont(state),
            }
        } else {
            flow::cont(state)
        }
    }
}
