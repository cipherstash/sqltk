use std::{any::Any, convert::Infallible};

use sqlparser::ast::{Query, Statement};
use sqltk::{flow, Node, Visitor, VisitorControlFlow};

use crate::model::ScopeOps;

/// `StackManager` is a [`Visitor`] implementation that ensures that new lexical
/// scope stack frames are pushed on entering statements and subqueries and
/// popped when exiting.
pub struct UpdateStack;

impl UpdateStack {
    fn should_update_stack<N: 'static>(node: &N) -> bool {
        (node as &dyn Any).downcast_ref::<Statement>().is_some()
            || (node as &dyn Any).downcast_ref::<Query>().is_some()
    }
}

impl<'ast, State> Visitor<'ast, State, Infallible> for UpdateStack
where
    State: ScopeOps<'ast>,
{
    fn enter<N: 'static>(&self, node: &'ast N, mut state: State) -> VisitorControlFlow<'ast, State, Infallible>
    where
        &'ast N: Into<Node<'ast>>,
    {
        if Self::should_update_stack(node) {
            state.push_scope()
        }
        flow::cont(state)
    }

    fn exit<N: 'static>(&self, node: &'ast N, mut state: State) -> VisitorControlFlow<'ast, State, Infallible>
    where
        &'ast N: Into<Node<'ast>>,
    {
        if Self::should_update_stack(node) {
            state.pop_scope()
        }
        flow::cont(state)
    }
}
