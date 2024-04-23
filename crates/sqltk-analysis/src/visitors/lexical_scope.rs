//! Types and traits for maintaining a lexical scope during AST traversal and
//! resolving identifiers within it.

use core::{marker::PhantomData, ops::Deref};
use std::rc::Rc;

use sqlparser::ast::{Cte, Ident, SetExpr, Statement, TableAlias, TableFactor};
use sqltk::{
    flow,
    prelude::{Expr, Query},
    Visitable, Visitor,
};
use unicase::UniCase;

use crate::{
    annotations::Annotates,
    model::{
        resolution_error::ResolutionError,
        source_annotation::{NamedRelation, SourceAnnotation},
    },
    node_path::NodePathOps,
    projection_annotation::{Projection, ProjectionAnnotation},
    schema_api::SchemaOps,
};

/// Operations for manipulating lexical scope and resolving identifiers that are in-scope.
pub trait LexicalScopeOps<'ast> {
    /// Resets the scope stack. This is called between every `Statement`.
    fn reset_scope(&mut self);

    /// Pushes a new empty scope onto the stack.
    fn push_scope(&mut self);

    /// Pops the top scope off the stack.
    fn pop_scope(&mut self);

    /// Puts a table/view/subquery projection into scope.
    fn add_relation(&mut self, relation: NamedRelation) -> Result<NamedRelation, ResolutionError>;

    /// Resolves a relation that is either in-scope (such as an aliased subquery or CTE)
    /// falling back to the database schema.
    fn resolve_relation(
        &mut self,
        name: &UniCase<String>,
    ) -> Result<NamedRelation, ResolutionError>;

    /// Resolves an identifier within the current scope.
    fn resolve_ident(&self, ident: &'ast Ident) -> Result<Rc<SourceAnnotation>, ResolutionError>;

    /// Resolves a compound identifier within the current scope.
    fn resolve_compound_ident(
        &self,
        ident: &'ast [Ident],
    ) -> Result<Rc<SourceAnnotation>, ResolutionError>;

    /// Expands a wildcard within the current scope.
    fn resolve_wildcard(&self) -> Result<Projection, ResolutionError>;

    /// Expands a qualified wildcard within the current scope.
    fn resolve_qualified_wildcard(
        &self,
        ident: &'ast [Ident],
    ) -> Result<Projection, ResolutionError>;
}

/// Provides methods that build [`Visitor`] implementations for managing the
/// lexical scope during AST traversal.
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct LexicalScope<'ast, State: 'ast>(PhantomData<&'ast State>);

impl<'ast, State: 'ast> LexicalScope<'ast, State>
where
    State: LexicalScopeOps<'ast>
        + Annotates<'ast, Expr, SourceAnnotation>
        + Annotates<'ast, SetExpr, ProjectionAnnotation>
        + Annotates<'ast, Query, ProjectionAnnotation>
        + NodePathOps<'ast>
        + SchemaOps,
{
    pub fn push_and_pop_scope_for_subqueries() -> impl Visitor<'ast, State> {
        Query::on_enter_exit(
            flow::modify_state(|state: &mut State| state.push_scope()),
            flow::modify_state(|state: &mut State| state.pop_scope()),
        )
    }

    pub fn push_and_pop_scope_for_statements() -> impl Visitor<'ast, State> {
        Statement::on_enter_exit(
            flow::modify_state(|state: &mut State| state.push_scope()),
            flow::modify_state(|state: &mut State| state.pop_scope()),
        )
    }

    pub fn bring_cte_into_scope() -> impl Visitor<'ast, State, ResolutionError> {
        Cte::on_exit(|node, state: State| {
            let mut state = state;
            let Cte {
                alias:
                    TableAlias {
                        name: alias,
                        columns,
                    },
                query,
                ..
            } = node;

            if !columns.is_empty() {
                return flow::error(ResolutionError::Unimplemented, state);
            }

            match state.expect_annotation(query.deref()) {
                Ok(projection_annotation) => match projection_annotation.deref() {
                    ProjectionAnnotation::Query(projection) => {
                        match state.add_relation(NamedRelation {
                            name: UniCase::new(alias.to_string()),
                            projection: projection.clone(),
                        }) {
                            Ok(_) => flow::cont(state),
                            Err(err) => flow::error(err, state),
                        }
                    }
                    _ => flow::error(ResolutionError::Unimplemented, state),
                },
                Err(err) => flow::error(err.into(), state),
            }
        })
    }

    pub fn bring_table_factor_into_scope() -> impl Visitor<'ast, State, ResolutionError> {
        TableFactor::on_exit(|node, state: State| {
            let mut state = state;
            match node {
                TableFactor::Table {
                    name,
                    alias,
                    args: None,
                    with_hints,
                    version: None,
                    partitions,
                } if with_hints.is_empty() && partitions.is_empty() => {
                    let name = UniCase::from(name.0.last().unwrap().value.clone());
                    let result = state.resolve_relation(&name).or_else(|_| {
                        state
                            .get_schema()
                            .resolve_table(&name)
                            .ok_or(ResolutionError::NoSuchRelation(name.to_string()))
                            .and_then(|table| {
                                state.add_relation(NamedRelation {
                                    name: local_table_identifier(name, alias),
                                    projection: Projection::from(&table),
                                })
                            })
                    });

                    match result {
                        Ok(_) => flow::cont(state),
                        Err(err) => flow::error(err, state),
                    }
                }
                TableFactor::Derived {
                    lateral: _,
                    subquery,
                    alias: Some(alias),
                } => {
                    let projection_annotation: Result<Rc<ProjectionAnnotation>, _> =
                        state.expect_annotation(subquery.body.deref());

                    let result = projection_annotation
                        .map(|projection_annotation| projection_annotation.projection().cloned())
                        .map(|projection| match projection {
                            Some(projection) => state.add_relation(NamedRelation {
                                name: UniCase::new(alias.name.value.to_string()),
                                projection: projection.clone(),
                            }),
                            None => Err(ResolutionError::EmptyProjection(subquery.clone())),
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
        })
    }
}

fn local_table_identifier(name: UniCase<String>, alias: &Option<TableAlias>) -> UniCase<String> {
    match alias {
        Some(TableAlias {
            name: alias,
            columns,
        }) if columns.is_empty() => UniCase::from(alias.to_string()),
        // TODO: handle the other TableAlias variants
        _ => UniCase::from(name.to_string()),
    }
}
