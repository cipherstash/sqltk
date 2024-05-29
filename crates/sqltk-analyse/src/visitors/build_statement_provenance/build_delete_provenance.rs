use std::{marker::PhantomData, ops::ControlFlow, rc::Rc};

use sqlparser::ast::{
    Delete, FromTable, Query, SelectItem, Statement, TableAlias, TableFactor, TableWithJoins,
};
use sqltk::{visitor_extensions::VisitorExtensions, Break, Visitable, Visitor};

use crate::{
    Annotate, AnnotateMut, DeleteProvenance, Projection, Provenance, ResolutionError, SchemaOps,
    ScopeOps, SqlIdent,
};

#[derive(Debug)]
pub struct BuildDeleteProvenance<'ast, State>(PhantomData<&'ast ()>, PhantomData<State>)
where
    State: SchemaOps
        + Annotate<'ast, Statement, Provenance>
        + Annotate<'ast, Query, Projection>
        + Annotate<'ast, Vec<SelectItem>, Projection>
        + ScopeOps;

impl<'ast, State> Default for BuildDeleteProvenance<'ast, State>
where
    State: SchemaOps
        + Annotate<'ast, Statement, Provenance>
        + Annotate<'ast, Query, Projection>
        + Annotate<'ast, Vec<SelectItem>, Projection>
        + ScopeOps,
{
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<'ast, State> Visitor<'ast> for BuildDeleteProvenance<'ast, State>
where
    State: SchemaOps
        + AnnotateMut<'ast, Statement, Provenance>
        + Annotate<'ast, Query, Projection>
        + Annotate<'ast, Vec<SelectItem>, Projection>
        + ScopeOps,
{
    type Error = ResolutionError;
    type State = State;

    fn exit<N: Visitable<'ast>>(
        &self,
        node: &'ast N,
        mut state: State,
    ) -> ControlFlow<Break<State, ResolutionError>, State> {
        if let Some(statement) = node.downcast_ref::<Statement>() {
            match statement {
                // For reference, the Postgres DELETE grammar can be found here:
                // https://github.com/postgres/postgres/blob/86a2d2a321215797abd1c67d9f2c52510423a97a/src/backend/parser/gram.y#L12327
                Statement::Delete(Delete {
                    // The table we are deleting rows from.
                    from,
                    // The RETURNING clause.
                    returning,
                    // USING (if present) will bring relations into scope that
                    // can be referenced in the RETURNING clause.
                    using: _,
                    // IGNORED (This is the WHERE clause and is not (currently)
                    // relevant to building provenance)
                    selection: _,
                    // IGNORED (MySQL allows deleting from multiple tables at
                    // once)
                    tables: _,
                    // IGNORED (MySQL supports ORDER BY and LIMIT in a DELETE)
                    order_by: _,
                    // IGNORED (MySQL supports ORDER BY and LIMIT in a DELETE)
                    limit: _,
                }) => {
                    let tables: &Vec<TableWithJoins> = match from {
                        FromTable::WithFromKeyword(tables) | FromTable::WithoutKeyword(tables) => {
                            tables
                        }
                    };

                    // Postgres does not support more than one table in the FROM clause.
                    let table_factor = if tables.len() == 1 {
                        &tables[0].relation
                    } else {
                        return self
                            .break_with_error(ResolutionError::TooManyTablesInDeleteFromClause);
                    };

                    match table_factor {
                        TableFactor::Table {
                            name,
                            alias,
                            args: None,
                            with_hints,
                            version: None,
                            partitions,
                        } if with_hints.is_empty() && partitions.is_empty() => {
                            let alias = match alias {
                                Some(TableAlias { name, columns }) if columns.is_empty() => {
                                    Some(name)
                                }
                                None => None,
                                _ => {
                                    return self.break_with_error(
                                        ResolutionError::UnsupportTableAliasVariant,
                                    )
                                }
                            };

                            let from_table_ident = alias
                                .map(SqlIdent::from)
                                .unwrap_or_else(|| SqlIdent::from(name.0.last().unwrap()));

                            match state.get_schema().resolve_table(&from_table_ident) {
                                Ok(from_table) => {
                                    // The `RETURNING` clause is a list of
                                    // expressions that are resolved with the
                                    // `FROM` table and relations brought into
                                    // scope by the `USING` clause.
                                    let returning: Result<Option<Rc<Projection>>, _> = returning
                                        .as_ref()
                                        .map(|items| state.get_annotation(items))
                                        .transpose()
                                        .map_err(ResolutionError::from);

                                    match returning {
                                        Ok(returning) => {
                                            state.set_annotation(
                                                statement,
                                                Provenance::Delete(
                                                    DeleteProvenance {
                                                        from_table,
                                                        returning,
                                                    }
                                                    .into(),
                                                ),
                                            );
                                            self.continue_with_state(state)
                                        }
                                        Err(err) => self.break_with_error(err),
                                    }
                                }
                                Err(err) => self.break_with_error(err.into()),
                            }
                        }
                        _ => self.break_with_error(
                            ResolutionError::UnsupportedTableFactorVariantInDelete,
                        ),
                    }
                }
                _ => self.continue_with_state(state),
            }
        } else {
            self.continue_with_state(state)
        }
    }
}
