use std::{marker::PhantomData, ops::Deref};

use sqlparser::ast::{Query, SetExpr};
use sqltk::{flow, Visitable, Visitor, VisitorControlFlow};

use crate::{
    model::{Annotate, Projection, ResolutionError, ScopeOps},
    SchemaOps,
};

#[derive(Debug)]
pub struct BuildQueryProjection<'ast, State>(PhantomData<&'ast ()>, PhantomData<State>);

impl<'ast, State> Default for BuildQueryProjection<'ast, State>
where
    State: ScopeOps
        + Annotate<'ast, Query, Projection>
        + Annotate<'ast, SetExpr, Projection>
        + SchemaOps,
{
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<'ast, State> Visitor<'ast> for BuildQueryProjection<'ast, State>
where
    State: ScopeOps
        + Annotate<'ast, Query, Projection>
        + Annotate<'ast, SetExpr, Projection>
        + SchemaOps,
{
    type Error = ResolutionError;
    type State = State;

    fn exit<N: Visitable<'ast>>(
        &self,
        node: &'ast N,
        mut state: State,
    ) -> VisitorControlFlow<'ast, State, ResolutionError> {
        if let Some(query) = node.downcast_ref::<Query>() {
            match state.expect_annotation(query.body.deref()) {
                Ok(projection_annotation) => {
                    state.add_annotation(query, projection_annotation);
                    flow::cont(state)
                }
                Err(err) => flow::error(err.into()),
            }
        } else {
            flow::cont(state)
        }
    }
}
