use std::{marker::PhantomData, rc::Rc};

use sqlparser::ast::{Expr, SelectItem};
use sqltk::{flow, prelude::VisitorControlFlow, Visitable, Visitor};

use crate::{Annotate, AnnotateMut, ProjectionColumn, ResolutionError, ScopeOps, Source, SqlIdent};

#[derive(Debug)]
pub struct BuildProjectionColumns<'ast, State>(PhantomData<&'ast ()>, PhantomData<State>);

impl<'ast, State> Default for BuildProjectionColumns<'ast, State>
where
    State: ScopeOps
        + Annotate<'ast, Expr, Source>
        + AnnotateMut<'ast, SelectItem, Vec<Rc<ProjectionColumn>>>,
{
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<'ast, State> Visitor<'ast> for BuildProjectionColumns<'ast, State>
where
    State: ScopeOps
        + Annotate<'ast, Expr, Source>
        + AnnotateMut<'ast, SelectItem, Vec<Rc<ProjectionColumn>>>,
{
    type State = State;
    type Error = ResolutionError;

    fn exit<N: Visitable<'ast>>(
        &self,
        node: &'ast N,
        mut state: State,
    ) -> VisitorControlFlow<'ast, State, ResolutionError> {
        if let Some(node) = node.downcast_ref::<SelectItem>() {
            let result: Vec<Result<Rc<ProjectionColumn>, ResolutionError>> = match node {
                SelectItem::UnnamedExpr(expr @ Expr::Identifier(ident)) => vec![state
                    .get_annotation(expr)
                    .map(|source: Rc<Source>| {
                        ProjectionColumn::new(source.clone(), Some(SqlIdent::from(ident).into()))
                            .into()
                    })
                    .map_err(ResolutionError::from)],
                SelectItem::UnnamedExpr(expr @ Expr::CompoundIdentifier(idents)) => vec![state
                    .get_annotation(expr)
                    .map(|source: Rc<Source>| {
                        ProjectionColumn::new(
                            source.clone(),
                            Some(SqlIdent::from(idents.last().unwrap()).into()),
                        )
                        .into()
                    })
                    .map_err(ResolutionError::from)],
                SelectItem::UnnamedExpr(expr) => vec![state
                    .get_annotation(expr)
                    .map(|source: Rc<Source>| ProjectionColumn::new(source.clone(), None).into())
                    .map_err(ResolutionError::from)],
                SelectItem::ExprWithAlias { expr, alias } => vec![state
                    .get_annotation(expr)
                    .map(|source: Rc<Source>| {
                        ProjectionColumn::new(source.clone(), Some(SqlIdent::from(alias).into()))
                            .into()
                    })
                    .map_err(ResolutionError::from)],
                SelectItem::QualifiedWildcard(obj_name, _) => {
                    let idents: Vec<SqlIdent> = obj_name.0.iter().map(SqlIdent::from).collect();
                    match state.resolve_qualified_wildcard(idents.as_slice()) {
                        Ok(projection) => Vec::from_iter(projection.columns_iter().map(Ok)),
                        Err(err) => vec![Err(err)],
                    }
                }
                SelectItem::Wildcard(_) => match state.resolve_wildcard() {
                    Ok(projection) => {
                        Vec::from_iter(projection.columns_iter().map(|c| Ok(c.clone())))
                    }
                    Err(err) => vec![Err(err)],
                },
            };

            let result: Result<Vec<Rc<ProjectionColumn>>, ResolutionError> =
                result.into_iter().collect();

            match result {
                Ok(columns) => {
                    state.set_annotation(node, columns);
                    flow::cont(state)
                }
                Err(err) => flow::error(err),
            }
        } else {
            flow::cont(state)
        }
    }
}
