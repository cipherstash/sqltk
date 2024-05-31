use std::{marker::PhantomData, ops::ControlFlow, rc::Rc};

use sqlparser::ast::{Expr, SelectItem};
use sqltk::{visitor_extensions::VisitorExtensions, Break, Visitable, Visitor};

use crate::{
    Annotate, AnnotateMut, ColumnWithOptionalAlias, ExprSource, ResolutionError, ScopeOps,
    SelectItemSource, SqlIdent,
};

#[derive(Debug)]
pub struct BuildSelectItemSources<'ast, State>(PhantomData<&'ast ()>, PhantomData<State>);

impl<'ast, State> Default for BuildSelectItemSources<'ast, State>
where
    State: ScopeOps
        + Annotate<'ast, Expr, ExprSource>
        + AnnotateMut<'ast, SelectItem, SelectItemSource>,
{
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<'ast, State> Visitor<'ast> for BuildSelectItemSources<'ast, State>
where
    State: ScopeOps
        + Annotate<'ast, Expr, ExprSource>
        + AnnotateMut<'ast, SelectItem, SelectItemSource>,
{
    type State = State;
    type Error = ResolutionError;

    fn exit<N: Visitable<'ast>>(
        &self,
        node: &'ast N,
        mut state: State,
    ) -> ControlFlow<Break<State, ResolutionError>, State> {
        if let Some(node) = node.downcast_ref::<SelectItem>() {
            let result: Result<Rc<SelectItemSource>, ResolutionError> = match node {
                SelectItem::UnnamedExpr(expr @ Expr::Identifier(ident)) => state
                    .get_annotation(expr)
                    .map(|source: Rc<ExprSource>| {
                        SelectItemSource::ColumnWithOptionalAlias(
                            ColumnWithOptionalAlias::new(
                                source.clone(),
                                Some(SqlIdent::from(ident).into()),
                            )
                            .into(),
                        )
                        .into()
                    })
                    .map_err(ResolutionError::from),
                SelectItem::UnnamedExpr(expr @ Expr::CompoundIdentifier(idents)) => state
                    .get_annotation(expr)
                    .map(|source: Rc<ExprSource>| {
                        SelectItemSource::ColumnWithOptionalAlias(
                            ColumnWithOptionalAlias::new(
                                source.clone(),
                                Some(SqlIdent::from(idents.last().unwrap()).into()),
                            )
                            .into(),
                        )
                        .into()
                    })
                    .map_err(ResolutionError::from),
                SelectItem::UnnamedExpr(expr) => state
                    .get_annotation(expr)
                    .map(|source: Rc<ExprSource>| {
                        SelectItemSource::ColumnWithOptionalAlias(
                            ColumnWithOptionalAlias::new(source.clone(), None).into(),
                        )
                        .into()
                    })
                    .map_err(ResolutionError::from),
                SelectItem::ExprWithAlias { expr, alias } => state
                    .get_annotation(expr)
                    .map(|source: Rc<ExprSource>| {
                        SelectItemSource::ColumnWithOptionalAlias(
                            ColumnWithOptionalAlias::new(
                                source.clone(),
                                Some(SqlIdent::from(alias).into()),
                            )
                            .into(),
                        )
                        .into()
                    })
                    .map_err(ResolutionError::from),
                SelectItem::QualifiedWildcard(obj_name, _) => {
                    let idents: Vec<SqlIdent> = obj_name.0.iter().map(SqlIdent::from).collect();
                    match state.resolve_qualified_wildcard(idents.as_slice()) {
                        Ok(columns) => {
                            Ok(SelectItemSource::ResolvedWildcard(columns.clone()).into())
                        }
                        Err(err) => Err(err),
                    }
                }
                SelectItem::Wildcard(_) => match state.resolve_wildcard() {
                    Ok(columns) => Ok(SelectItemSource::ResolvedWildcard(columns.clone()).into()),
                    Err(err) => Err(err),
                },
            };

            match result {
                Ok(columns) => {
                    state.set_annotation(node, columns);
                    self.continue_with_state(state)
                }
                Err(err) => self.break_with_error(err),
            }
        } else {
            self.continue_with_state(state)
        }
    }
}
