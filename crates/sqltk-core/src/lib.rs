mod transform;
mod transformable_impls;
mod visitable_impls;

pub mod visitor_extensions;
pub use transform::*;

// Re-export sqlparser
pub use sqlparser;
// Re-export bigdecimal
pub use bigdecimal;

use core::fmt::Debug;
use std::{any::Any, error::Error, ops::ControlFlow};

/// Trait for types that can visit any `sqlparser` AST node.
///
/// A "visit" of a single AST node is a call to `enter` followed some time later
/// with a call to `exit` if no error was returned by `enter`.
///
/// Visitors are parameterised by a `State` and an error `E`. If unspecified,
/// `E` is [`Infallible`] which means an error cannot be returned by `enter` or
/// `exit`.
pub trait Visitor<'ast> {
    type State;
    type Error: Error + Debug;

    /// Called when node is entered.
    ///
    /// `node` is a reference to any `sqlparser` AST node type.
    ///
    /// Ownership of `state` is passed to `enter`. Typically `state` will be
    /// mutated and returned to the caller in the [`VisitorControlFlow`] value,
    /// but the implementation is free to drop the original state and return a
    /// new value should it wish to do so.
    fn enter<N: Visitable<'ast>>(
        &self,
        _node: &'ast N,
        state: Self::State,
    ) -> ControlFlow<Break<Self::State, Self::Error>, Self::State> {
        ControlFlow::Continue(state)
    }

    /// Called when node is exited.
    ///
    /// `node` is a reference to any `sqlparser` AST node type.
    ///
    /// Ownership of `state` is passed to `exit`. Typically `state` will be
    /// mutated and returned to the caller in the [`VisitorControlFlow`] value,
    /// but the implementation is free to drop the original state and return a
    /// new value should it wish to do so.
    fn exit<N: Visitable<'ast>>(
        &self,
        _node: &'ast N,
        state: Self::State,
    ) -> ControlFlow<Break<Self::State, Self::Error>, Self::State> {
        ControlFlow::Continue(state)
    }
}

/// Trait for types that can be visited by a [`Visitor`].
/// This trait is implemented for all AST nodes defined by `sqlparser`.
pub trait Visitable<'ast>
where
    Self: 'static + Sized,
{
    /// Entry point to begin AST traversal with a [`Visitor`].
    ///
    /// Invokes [`Visitor::enter`] and [`Visitor::exit`] for
    /// every AST node encountered during traversal with the exception that if
    /// the `Visitor` returns
    /// [`VisitorControlFlow::Continue(Nav::Skip)`], remaining children of the
    /// current node will be skipped.
    fn accept<V>(
        &'ast self,
        visitor: &V,
        state: V::State,
    ) -> ControlFlow<Break<V::State, V::Error>, V::State>
    where
        V: Visitor<'ast>;

    /// Traverses the AST invoking the `enter` & `exit` methods on `visitor`
    /// in order to compute a new state.
    ///
    /// Returns `Ok(state)` on success, or `Err((err, state))` if an error
    /// occurs.
    fn evaluate<V>(&'ast self, visitor: &V, state: V::State) -> Result<V::State, V::Error>
    where
        V: Visitor<'ast>,
    {
        match self.accept(visitor, state) {
            ControlFlow::Continue(state) => Ok(state),
            ControlFlow::Break(brk) => match brk {
                Break::Finished(state) => Ok(state),
                Break::SkipChildren(state) => Ok(state),
                Break::Err(err) => Err(err),
            },
        }
    }

    /// Convenience for downcasting a `self` to a concrete [`Visitable`].
    fn downcast_ref<To: Visitable<'ast>>(&'ast self) -> Option<&'ast To> {
        (self as &dyn Any).downcast_ref::<To>()
    }

    /// Convenience for downcasting a `self` to a concrete mutatble [`Visitable`].
    fn downcast_mut<To: Visitable<'ast>>(&'ast mut self) -> Option<&'ast mut To> {
        (self as &mut dyn Any).downcast_mut::<To>()
    }

    /// Convenience for testing if `self` is a specific [`Visitable`] type.
    fn is<T: Visitable<'ast>>(&'ast self) -> bool {
        (self as &dyn Any).is::<T>()
    }
}

/// Type used to signal abnormal control flow from a [`Visitor`] during
/// AST traversal.
///
/// All variants propagate the `State` back up the call stack.
#[derive(Debug)]
pub enum Break<State, E> {
    /// Do not visit child nodes of the current node and resume traversal
    /// from the next sibling of the current node. This is useful to save CPU
    /// cycles when an exhaustive traversal of an AST is not required.
    ///
    /// This variant has no effect when returned from [`Visitor::exit`]; instead
    /// it is treated the same as `Continue`.
    SkipChildren(State),

    /// Ends traversal entirely but completes successfully. Useful to force an
    /// early return when an exhaustive traversal of the AST is unnecessary.
    /// An example use-case would be implementing a search over the AST that
    /// returns as soon as the target has been found.
    Finished(State),

    /// An error occurred. Traversal will be aborted.
    Err(E),
}

impl<State, E> PartialEq for Break<State, E>
where
    State: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::SkipChildren(l0), Self::SkipChildren(r0)) => l0 == r0,
            (Self::Finished(l0), Self::Finished(r0)) => l0 == r0,
            (Self::Err(_), Self::Err(_)) => false,
            _ => false,
        }
    }
}
