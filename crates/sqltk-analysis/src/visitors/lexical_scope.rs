//! Types and traits for maintaining a lexical scope during AST traversal and
//! resolving identifiers within it.

use core::{convert::Infallible, marker::PhantomData, ops::Deref};

use sqlparser::ast::{Ident, SelectItem, SetExpr, Statement, TableAlias, TableFactor};
use sqltk::{flow, prelude::Expr, Visitable, Visitor, VisitorControlFlow};

use crate::{
    annotate_sources::SourceAnnotationOps,
    model::resolution_error::ResolutionError,
    model::sources::{Relation, Source},
    schema_api::SchemaOps,
    sources::ColumnRef,
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
    fn add_relation(&mut self, relation: Relation) -> Result<Relation, ResolutionError>;

    /// Resolves an identifier within the current scope.
    fn resolve_ident(&self, ident: &'ast Ident) -> Result<Source, ResolutionError>;

    /// Resolves a compound identifier within the current scope.
    fn resolve_compound_ident(&self, ident: &'ast [Ident]) -> Result<Source, ResolutionError>;

    /// Expands a wildcard within the current scope.
    fn resolve_wildcard(&self) -> Result<Vec<Source>, ResolutionError>;

    /// Expands a qualified wildcard within the current scope.
    fn resolve_qualified_wildcard(
        &self,
        ident: &'ast [Ident],
    ) -> Result<Vec<Source>, ResolutionError>;
}

/// Provides methods that build [`Visitor`] implementations for managing the
/// lexical scope during AST traversal.
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct LexicalScope<'ast, State: 'ast>(PhantomData<&'ast State>);

impl<'ast, State: 'ast> LexicalScope<'ast, State>
where
    State: LexicalScopeOps<'ast> + SourceAnnotationOps<'ast> + SchemaOps,
{
    /// Builds a `Visitor` that resets the lexical scope on enter and exit of
    /// every [`Statement`] node.
    pub fn reset_for_each_statement() -> impl Visitor<'ast, State, Infallible> {
        Statement::on_enter_exit(&Self::reset_scope, &Self::reset_scope)
    }

    /// Builds a `Visitor` that brings tables/views/subqueries into scope.
    pub fn bring_tables_into_scope() -> impl Visitor<'ast, State, ResolutionError> {
        // This used to be on_enter
        TableFactor::on_exit(&Self::push_table_factor_onto_scope)
    }

    fn push_table_factor_onto_scope(
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
                let name: &Ident = name.0.last().unwrap();
                let result = state
                    .get_schema()
                    .resolve_table(&name.to_string())
                    .ok_or(ResolutionError::NoSuchIdentifier(name.to_string()))
                    .and_then(|table| {
                        table_alias(alias).and_then(|alias| {
                            state.add_relation(Relation::TableLike(table.clone(), alias))
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
                alias,
            } => {
                let query = subquery.deref();
                #[allow(unused_variables)]
                match query.body.deref() {
                    SetExpr::Select(select) => {
                        let projection_sources: Result<Vec<_>, _> = select
                            .projection
                            .iter()
                            .map(|item| state.get_source(item).map(|source| (item, source.clone())))
                            .enumerate()
                            .map(|(idx, result)| match result {
                                Ok((SelectItem::UnnamedExpr(expr), source)) => {
                                    if let Expr::Identifier(ident) = expr {
                                        Ok((ColumnRef::Identifier(ident.to_string()), source))
                                    } else {
                                        Ok((ColumnRef::Ordinal(idx), source))
                                    }
                                }
                                Ok((SelectItem::ExprWithAlias { expr: _, alias }, source)) => {
                                    Ok((ColumnRef::Identifier(alias.to_string()), source))
                                }
                                Ok((SelectItem::QualifiedWildcard(_, _), source)) => {
                                    todo!("SelectItem::QualifiedWildcard")
                                }
                                Ok((SelectItem::Wildcard(_), source)) => {
                                    todo!("SelectItem::Wildcard")
                                }
                                Err(err) => Err(err),
                            })
                            .collect();

                        match projection_sources {
                            Ok(projection_sources) => {
                                match table_alias(alias).map(|alias| {
                                    state.add_relation(Relation::SubQuery(
                                        query.clone(),
                                        projection_sources,
                                        alias,
                                    ))
                                }) {
                                    Ok(_) => flow::cont(state),
                                    Err(err) => flow::error(err, state),
                                }
                            }
                            Err(err) => flow::error(err, state),
                        }
                    }
                    SetExpr::Query(_) => todo!("SetExpr::Query"),
                    SetExpr::SetOperation {
                        op,
                        set_quantifier,
                        left,
                        right,
                    } => todo!("SetExpr::Query"),
                    SetExpr::Values(_) => todo!("SetExpr::Values"),
                    SetExpr::Insert(_) => todo!("SetExpr::Insert"),
                    SetExpr::Update(_) => todo!("SetExpr::Update"),
                    SetExpr::Table(_) => todo!("SetExpr::Table"),
                }
            }
            #[allow(unused_variables)]
            TableFactor::TableFunction { expr, alias } => todo!("TableFactor::TableFunction"),
            #[allow(unused_variables)]
            TableFactor::Function {
                lateral,
                name,
                args,
                alias,
            } => todo!("TableFactor::Function"),
            #[allow(unused_variables)]
            TableFactor::UNNEST {
                alias,
                array_exprs,
                with_offset,
                with_offset_alias,
            } => todo!("TableFactor::UNNEST"),
            #[allow(unused_variables)]
            TableFactor::JsonTable {
                json_expr,
                json_path,
                columns,
                alias,
            } => todo!("TableFactor::JsonTable"),
            #[allow(unused_variables)]
            TableFactor::NestedJoin {
                table_with_joins,
                alias,
            } => todo!("TableFactor::NestedJoin"),
            #[allow(unused_variables)]
            TableFactor::Pivot {
                table,
                aggregate_function,
                value_column,
                pivot_values,
                alias,
            } => todo!("TableFactor::Pivot"),
            #[allow(unused_variables)]
            TableFactor::Unpivot {
                table,
                value,
                name,
                columns,
                alias,
            } => todo!("TableFactor::Unpivot"),
            _ => unimplemented!("WAT"),
        }
    }

    fn reset_scope(
        _: &'ast Statement,
        mut state: State,
    ) -> VisitorControlFlow<'ast, State, Infallible> {
        state.reset_scope();
        flow::infallible::cont(state)
    }
}

pub fn reset_scope<'ast, State>(
    _: &'ast Statement,
    mut state: State,
) -> VisitorControlFlow<'ast, State, ResolutionError>
where
    State: LexicalScopeOps<'ast>,
{
    state.reset_scope();
    flow::cont(state)
}

fn table_alias(alias: &Option<TableAlias>) -> Result<Option<String>, ResolutionError> {
    match alias {
        Some(TableAlias { name, columns }) if columns.is_empty() => {
            Ok(Some(name.to_string().clone()))
        }
        Some(alias) => unimplemented!("Unhandled TableAlias variant: {:?}", alias),
        None => Ok(None),
    }
}
