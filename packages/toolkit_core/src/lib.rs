mod display_type_name;
mod generated;
mod visitable_impls;

// Re-export sqlparser
pub use sqlparser;
// Re-export bigdecimal
pub use bigdecimal;

pub use dispatch::*;
pub use display_type_name::*;
pub use generated::sql_node::*;

pub mod annotation;
pub mod dispatch;
pub mod pipeline;
pub use pipeline::*;

/// Exposed to allow macro-generated code and some internals to work.
#[doc(hidden)]
pub mod private;

use private::visit;
use std::{error::Error, marker::PhantomData, ops::ControlFlow};

#[derive(Debug)]
pub enum Break<State> {
    SkipChildren(State),
    Abort(State),
    Err(Box<dyn Error>, State),
}

impl<State> PartialEq for Break<State>
where
    State: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::SkipChildren(l0), Self::SkipChildren(r0)) => l0 == r0,
            (Self::Abort(l0), Self::Abort(r0)) => l0 == r0,
            (Self::Err(_, _), Self::Err(_, _)) => false,
            _ => false,
        }
    }
}

impl<State> Eq for Break<State> where State: Eq {}

/// [`ControlFlow`] type alias for values returned by [`Visitor::enter`] &
/// [`Visitor::exit`].
pub type VisitorControlFlow<State> = ControlFlow<Break<State>, State>;

pub struct Flow<State>(PhantomData<State>);

impl<State> Flow<State> {
    pub fn cont(state: State) -> VisitorControlFlow<State> {
        VisitorControlFlow::Continue(state)
    }

    pub fn skip(state: State) -> VisitorControlFlow<State> {
        VisitorControlFlow::Break(Break::SkipChildren(state))
    }

    pub fn abort(state: State) -> VisitorControlFlow<State> {
        VisitorControlFlow::Break(Break::Abort(state))
    }

    pub fn error(error: Box<dyn Error>, state: State) -> VisitorControlFlow<State> {
        VisitorControlFlow::Break(Break::Err(error, state))
    }

    pub fn map_continue<F>(flow: VisitorControlFlow<State>, f: F) -> VisitorControlFlow<State>
    where
        F: FnOnce(State) -> VisitorControlFlow<State>,
    {
        match flow {
            VisitorControlFlow::Continue(state) => f(state),
            brk => brk,
        }
    }
}

/// Trait for types that visit a specific type of node.
#[allow(unused_variables)]
pub trait Visitor<'ast, Node, State>
where
    Node: Visitable<'ast>,
{
    /// Called when a node is entered.  The default implementation returns
    /// [`VisitorControlFlow::Continue(Nav::Visit)`].
    fn enter(&self, node: &'ast Node, state: State) -> VisitorControlFlow<State> {
        Flow::cont(state)
    }

    /// Called when a node is exited.  The default implementation returns
    /// [`VisitorControlFlow::Continue(())`].
    fn exit(&self, node: &'ast Node, state: State) -> VisitorControlFlow<State> {
        Flow::cont(state)
    }
}

/// Trait for types that can be visited by a [`VisitorDispatch`].
/// This trait is implemented for all AST nodes defined by `sqlparser`.
pub trait Visitable<'ast> {
    /// Entry point to begin AST traversal with a [`VisitorDispatch`].
    ///
    /// Invokes [`VisitorDispatch::enter`] and [`VisitorDispatch::exit`] for
    /// every AST node encountered during traversal with the exception that if
    /// the `VisitorDispatch` returns
    /// [`VisitorControlFlow::Continue(Nav::Skip)`], remaining children of the
    /// current node will be skipped.
    fn accept<State, VD: VisitorDispatch<'ast, State>>(
        &'ast self,
        dispatcher: &VD,
        state: State,
    ) -> VisitorControlFlow<State>;
}
