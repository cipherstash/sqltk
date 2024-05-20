use std::{marker::PhantomData, ops::Deref, rc::Rc};

use sqlparser::ast::{Expr, Select, SelectItem};
use sqltk::{flow, Visitable, Visitor, VisitorControlFlow};

use crate::{
    model::{Annotate, Projection, ResolutionError, ScopeOps, Source},
    ProjectionColumn, SchemaOps,
};

#[derive(Debug)]
pub struct BuildSelectProjection<'ast, State>(PhantomData<&'ast ()>, PhantomData<State>);

impl<'ast, State> Default for BuildSelectProjection<'ast, State>
where
    State: ScopeOps
        + Annotate<'ast, Expr, Source>
        + Annotate<'ast, Select, Projection>
        + Annotate<'ast, SelectItem, Vec<Rc<ProjectionColumn>>>
        + SchemaOps,
{
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<'ast, State> Visitor<'ast> for BuildSelectProjection<'ast, State>
where
    State: ScopeOps
        + Annotate<'ast, Expr, Source>
        + Annotate<'ast, Select, Projection>
        + Annotate<'ast, SelectItem, Vec<Rc<ProjectionColumn>>>
        + SchemaOps,
{
    type State = State;
    type Error = ResolutionError;

    fn exit<N: Visitable<'ast>>(
        &self,
        node: &'ast N,
        mut state: State,
    ) -> VisitorControlFlow<'ast, State, ResolutionError> {
        if let Some(select) = node.downcast_ref::<Select>() {
            let Select {
                projection: select_items,
                ..
            } = select;

            let result: Result<Vec<_>, _> = select_items
                .iter()
                .map(|item| state.expect_annotation(item))
                .collect();

            let result: Result<Vec<Rc<ProjectionColumn>>, _> = result.map(|items| {
                items
                    .iter()
                    .map(|item| item.deref())
                    .flatten()
                    .map(|item| (*item).clone())
                    .collect::<Vec<_>>()
            });

            match result {
                Ok(columns) => {
                    state.add_annotation(select, Projection::Columns(columns));
                    flow::cont(state)
                }
                Err(err) => flow::error(err.into()),
            }
        } else {
            flow::cont(state)
        }
    }
}
