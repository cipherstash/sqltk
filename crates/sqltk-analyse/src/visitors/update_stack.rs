use std::{convert::Infallible, marker::PhantomData};

use sqlparser::ast::{Query, Statement};
use sqltk::Visitor;

use crate::model::ScopeOps;

/// `UpdateStack` is a [`Visitor`] implementation that ensures that new lexical
/// scope frames are pushed on entering statements and subqueries and
/// popped when exiting statements and subqueries.
#[derive(Debug, Visitor)]
#[visitor(
    error_ty = Infallible,
    enter = {
        if node.is::<Statement>() || node.is::<Query>() {
            state.push_scope()
        }
        self.continue_with_state(state)
    },
    exit = {
        if node.is::<Statement>() || node.is::<Query>() {
            state.pop_scope()
        }
        self.continue_with_state(state)
    }
)]
pub struct UpdateStack<'ast, State>(PhantomData<&'ast ()>, PhantomData<State>)
where
    State: ScopeOps;

impl<'ast, State> Default for UpdateStack<'ast, State>
where
    State: ScopeOps,
{
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}
