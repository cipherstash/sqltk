use std::{
    marker::PhantomData,
    ops::{ControlFlow, Deref},
};

use sqlparser::ast::{Query, SetExpr};
use sqltk::{visitor_extensions::VisitorExtensions, Break, Visitable, Visitor};

use crate::{
    model::{Annotate, Projection, ResolutionError, ScopeOps},
    AnnotateMut, SchemaOps,
};

#[derive(Debug)]
pub struct BuildQueryProjection<'ast, State>(PhantomData<&'ast ()>, PhantomData<State>);

impl<'ast, State> Default for BuildQueryProjection<'ast, State>
where
    State: ScopeOps
        + AnnotateMut<'ast, Query, Projection>
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
        + AnnotateMut<'ast, Query, Projection>
        + Annotate<'ast, SetExpr, Projection>
        + SchemaOps,
{
    type Error = ResolutionError;
    type State = State;

    fn exit<N: Visitable<'ast>>(
        &self,
        node: &'ast N,
        mut state: State,
    ) -> ControlFlow<Break<State, ResolutionError>, State> {
        if let Some(query) = node.downcast_ref::<Query>() {
            match state.get_annotation(query.body.deref()) {
                Ok(projection_annotation) => {
                    state.set_annotation(query, projection_annotation);
                    self.continue_with_state(state)
                }
                Err(err) => self.break_with_error(err.into()),
            }
        } else {
            self.continue_with_state(state)
        }
    }
}
