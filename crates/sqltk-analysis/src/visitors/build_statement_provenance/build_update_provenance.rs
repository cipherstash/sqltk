use std::{marker::PhantomData, rc::Rc};

use sqlparser::ast::{Assignment, Expr, Query, SelectItem, Statement, TableFactor};
use sqltk::{
    prelude::{flow, VisitorControlFlow},
    Visitable, Visitor,
};

use crate::{
    Annotate, AnnotateMut, ColumnWritten, Projection, Provenance, ResolutionError, SchemaOps,
    SourceItem, SqlIdent, Table, UpdateProvenance,
};

#[derive(Debug)]
pub struct BuildUpdateProvenance<'ast, State>(PhantomData<&'ast ()>, PhantomData<State>);

impl<'ast, State> Default for BuildUpdateProvenance<'ast, State>
where
    State: SchemaOps
        + Annotate<'ast, Expr, SourceItem>
        + Annotate<'ast, Query, Projection>
        + Annotate<'ast, Vec<SelectItem>, Projection>
        + AnnotateMut<'ast, Statement, Provenance>,
{
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<'ast, State> Visitor<'ast> for BuildUpdateProvenance<'ast, State>
where
    State: SchemaOps
        + Annotate<'ast, Expr, SourceItem>
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
    ) -> VisitorControlFlow<'ast, State, ResolutionError> {
        if let Some(statement) = node.downcast_ref::<Statement>() {
            match statement {
                Statement::Update {
                    table,
                    assignments,
                    from: _,
                    selection: _,
                    returning,
                } => {
                    match &table.relation {
                        TableFactor::Table {
                            name: table_name, ..
                        } => {
                            let table: Result<Rc<Table>, _> = state
                                .get_schema()
                                .resolve_table(&SqlIdent::from(table_name.0.last().unwrap()))
                                .map_err(ResolutionError::from);

                            match table {
                                Ok(table) => {
                                    let columns_written: Result<Vec<ColumnWritten>, _> =
                                        assignments
                                            .iter()
                                            .map(|Assignment { id, value }| {
                                                // NOTE: columns cannot be assigned via an alias, BUT
                                                // assignments to fields of a column with a compound type are
                                                // permitted. We currently only validate that the column exists
                                                // - not any of the sub-fields - and we throw the sub-field
                                                // information away.
                                                let column = table
                                                    .get_column(&SqlIdent::from(
                                                        id.first().expect(
                                                            "column ident should have parsed",
                                                        ),
                                                    ))
                                                    .map_err(ResolutionError::from);

                                                let source = state.get_annotation(value);

                                                column
                                                    .and_then(|column| {
                                                        source
                                                            .map(|source| ColumnWritten {
                                                                column: column.name.clone(),
                                                                data: source,
                                                            })
                                                            .map_err(ResolutionError::from)
                                                    })
                                                    .map_err(ResolutionError::from)
                                            })
                                            .collect::<Result<Vec<_>, _>>();

                                    let returning: Result<Option<Rc<Projection>>, _> = returning
                                        .as_ref()
                                        .map(|items| state.get_annotation(items))
                                        .transpose()
                                        .map_err(ResolutionError::from);

                                    let result = columns_written.and_then(|columns_written| {
                                        returning.map(|returning| {
                                            state.set_annotation(
                                                statement,
                                                Provenance::Update(
                                                    UpdateProvenance {
                                                        update_table: table.clone(),
                                                        columns_written,
                                                        returning,
                                                    }
                                                    .into(),
                                                ),
                                            );
                                        })
                                    });

                                    match result {
                                        Ok(_) => flow::cont(state),
                                        Err(err) => flow::error(err),
                                    }
                                }
                                Err(err) => flow::error(err),
                            }
                        }
                        // `sqlparser` reuses certain AST types in grammar locations where
                        // not all of the variants of the type are permitted at that site.
                        // In this case, an UPDATE statement must refer to an actual table
                        // which mean only one variant of TableFactor is applicable.
                        //
                        // Using `unreachable!(..)` here is ill-advised, because without
                        // auditing the `sqlparser` code we cannot tell (without a thorough
                        // audit) if `sqlparser` will try to parse illegal variants. If we
                        // panic here, it would be a potential denial-of-service attack
                        // vector.
                        //
                        // See: https://www.postgresql.org/docs/current/sql-update.html
                        _ => flow::error(ResolutionError::InvalidAstNode),
                    }
                }
                _ => flow::cont(state),
            }
        } else {
            flow::cont(state)
        }
    }
}
