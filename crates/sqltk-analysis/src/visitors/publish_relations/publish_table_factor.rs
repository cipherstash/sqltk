use std::{ops::Deref, rc::Rc};

use sqlparser::ast::{Expr, Query, SetExpr, TableAlias, TableFactor};
use sqltk::{flow, SpecializedVisitor, VisitorControlFlow};

use crate::{
    model::Annotates,
    model::Projection,
    model::ResolutionError,
    model::ScopeOps,
    model::SqlIdent,
    model::{NamedRelation, Source},
    node_path::NodePathOps,
    SchemaOps,
};

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct PublishTableFactor;

impl<'ast, State: 'ast> SpecializedVisitor<'ast, TableFactor, State, ResolutionError>
    for PublishTableFactor
where
    State: ScopeOps<'ast>
        + Annotates<'ast, Expr, Source>
        + Annotates<'ast, SetExpr, Projection>
        + Annotates<'ast, Query, Projection>
        + NodePathOps<'ast>
        + SchemaOps,
{
    fn exit(
        &self,
        node: &'ast TableFactor,
        mut state: State,
    ) -> VisitorControlFlow<'ast, State, ResolutionError> {
        match node {
            TableFactor::Table {
                name,
                alias,
                args: None,
                with_hints,
                version: None,
                partitions,
            } if with_hints.is_empty() && partitions.is_empty() => {
                let name = SqlIdent::from(name.0.last().unwrap());
                let record_as = match alias {
                    Some(alias) => validate_table_alias(alias),
                    None => Ok(&name).cloned(),
                };

                let result = state.resolve_relation(&name).or_else(|_| {
                    state
                        .get_schema()
                        .resolve_table(&name)
                        .map_err(&ResolutionError::from)
                        .and_then(|table| {
                            record_as.and_then(|record_as| {
                                state.add_relation(
                                    NamedRelation {
                                        name: record_as.into(),
                                        projection: Rc::new(Projection::from(&table)),
                                    }
                                    .into(),
                                )
                            })
                        })
                });

                match result {
                    Ok(_) => flow::cont(state),
                    Err(err) => flow::error(err.into(), state),
                }
            }
            TableFactor::Derived {
                lateral: _,
                subquery,
                alias: Some(alias),
            } => {
                let projection: Result<Rc<Projection>, _> =
                    state.expect_annotation(subquery.body.deref());

                let result =
                    projection
                        .map(|projection| projection.deref().clone())
                        .map(|projection| {
                            state.add_relation(
                                NamedRelation {
                                    name: Rc::new(SqlIdent::from(&alias.name)),
                                    projection: projection.into(),
                                }
                                .into(),
                            )
                        });

                match result {
                    Ok(_) => flow::cont(state),
                    Err(err) => flow::error(err.into(), state),
                }
            }
            TableFactor::Derived {
                lateral: _,
                subquery: _,
                alias: None,
            } => flow::error(ResolutionError::Unimplemented, state),
            #[allow(unused_variables)]
            TableFactor::TableFunction { expr, alias } => {
                flow::error(ResolutionError::Unimplemented, state)
            }
            #[allow(unused_variables)]
            TableFactor::Function {
                lateral,
                name,
                args,
                alias,
            } => flow::error(ResolutionError::Unimplemented, state),
            #[allow(unused_variables)]
            TableFactor::UNNEST {
                alias,
                array_exprs,
                with_offset,
                with_offset_alias,
            } => flow::error(ResolutionError::Unimplemented, state),
            #[allow(unused_variables)]
            TableFactor::JsonTable {
                json_expr,
                json_path,
                columns,
                alias,
            } => flow::error(ResolutionError::Unimplemented, state),
            #[allow(unused_variables)]
            TableFactor::NestedJoin {
                table_with_joins,
                alias,
            } => flow::error(ResolutionError::Unimplemented, state),
            #[allow(unused_variables)]
            TableFactor::Pivot {
                table,
                aggregate_function,
                value_column,
                pivot_values,
                alias,
            } => flow::error(ResolutionError::Unimplemented, state),
            #[allow(unused_variables)]
            TableFactor::Unpivot {
                table,
                value,
                name,
                columns,
                alias,
            } => flow::error(ResolutionError::Unimplemented, state),
            _ => flow::error(ResolutionError::Unimplemented, state),
        }
    }
}

fn validate_table_alias(alias: &TableAlias) -> Result<SqlIdent, ResolutionError> {
    match alias {
        TableAlias { name, columns } if columns.is_empty() => Ok(SqlIdent::from(name)),
        _ => Err(ResolutionError::UnsupportTableAliasVariant),
    }
}
