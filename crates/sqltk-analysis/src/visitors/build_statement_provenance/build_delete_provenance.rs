#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use std::{marker::PhantomData, rc::Rc};

use sqlparser::ast::{
    Delete, FromTable, Query, SelectItem, Statement, TableAlias, TableFactor, TableWithJoins,
};
use sqltk::{
    prelude::{flow, VisitorControlFlow},
    Visitable, Visitor,
};

use crate::{
    Annotate, NamedRelation, Projection, Provenance, ResolutionError, SchemaOps, Scope, ScopeOps,
    SqlIdent,
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
        + Annotate<'ast, Statement, Provenance>
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
    ) -> VisitorControlFlow<'ast, State, ResolutionError> {
        if let Some(statement) = node.downcast_ref::<Statement>() {
            match statement {
                // For reference, the Postgres DELETE grammar can be found here:
                // https://github.com/postgres/postgres/blob/86a2d2a321215797abd1c67d9f2c52510423a97a/src/backend/parser/gram.y#L12327
                Statement::Delete(Delete {
                    from,
                    returning,
                    // USING clause. Not (currently) relevant to building provenance.
                    using: _,
                    // This is the WHERE clause and is not (currently) relevant to
                    // building provenance.
                    selection: _,
                    // MySQL allows deleting from multiple tables at once.
                    tables: _,
                    // MySQL supports ORDER BY and LIMIT in a DELETE.
                    order_by: _,
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
                        return flow::error(ResolutionError::TooManyTablesInDeleteFromClause);
                    };

                    match table_factor {
                        TableFactor::Table {
                            name,
                            alias,
                            args: None,
                            with_hints,
                            version: None,
                            partitions,
                        } if with_hints.len() == 0 && partitions.len() == 0 => {
                            let alias = match alias {
                                Some(TableAlias { name, columns }) if columns.len() == 0 => {
                                    Some(name)
                                }
                                None => None,
                                _ => {
                                    return flow::error(ResolutionError::UnsupportTableAliasVariant)
                                }
                            };

                            let from_table_ident = alias
                                .map(|alias| SqlIdent::from(alias))
                                .unwrap_or_else(|| SqlIdent::from(name.0.last().unwrap()));

                            match state.get_schema().resolve_table(&from_table_ident) {
                                Ok(from_table) => {
                                    // Resolve `returning` (which could be a
                                    // wildcard) within this table.  A new Scope is
                                    // created in which to resolve the columns in
                                    // `returning`.  This is because in a DELETE
                                    // statement only the columns from the table
                                    // whose rows are being deleted are allowed to
                                    // be returned, BUT any tables in the USING
                                    // clause (if any) will have been pushed into
                                    // scope and using a wildcard in RETURNING would
                                    // pick those up.
                                    //
                                    // So we ignore the existing lexical scope and
                                    // create a fresh scope containing only the FROM
                                    // table before resolving the `returning`
                                    // projection.
                                    let mut scope = Scope::default();
                                    scope.add_relation(Rc::new(NamedRelation::from(from_table)));
                                }
                                Err(err) => return flow::error(err.into()),
                            }
                        }
                        _ => {
                            return flow::error(
                                ResolutionError::UnsupportedTableFactorVariantInDelete,
                            )
                        }
                    }

                    todo!()
                }
                _ => flow::cont(state),
            }
        } else {
            flow::cont(state)
        }
    }
}
