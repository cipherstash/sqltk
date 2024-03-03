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
use std::ops::ControlFlow;

#[derive(Clone, Copy)]
/// Used as the "continue" type in the [`ControlFlow`] value returned by all
/// visitors.
pub enum Navigation {
    /// Skip visiting children of the current AST node
    Skip,
    /// Visit the children of the current AST node
    Visit,
}

/// [`ControlFlow`] type returned by [`Visitor::enter`].
pub type EnterControlFlow = ControlFlow<(), Navigation>;

/// [`ControlFlow`] type returned by [`Visitor::exit`].
pub type ExitControlFlow = ControlFlow<(), ()>;

/// Trait for types that visit a specific type of node.
#[allow(unused_variables)]
pub trait Visitor<N>
where
    N: Visitable,
{
    /// Called when a node is entered.
    ///
    /// The default implementation returns [`ControlFlow::Continue(Navigation::Visit)`].
    ///
    fn enter<'me, 'ast>(&'me mut self, node: &'ast N) -> EnterControlFlow
    where
        'ast: 'me,
    {
        ControlFlow::Continue(Navigation::Visit)
    }

    /// Called when a node is exited.  The default implementation returns
    /// [`ControlFlow::Continue(Navigation::Visit)`].  Note that the
    /// [`Navigation`] value returned in exit result is ignored.
    ///
    fn exit<'me, 'ast>(&'me mut self, node: &'ast N) -> ExitControlFlow
    where
        'ast: 'me,
    {
        ControlFlow::Continue(())
    }
}

/// Trait for types that can be visited by a [`VisitorDispatch`].
pub trait Visitable {
    /// Entry point to begin AST traversal with a [`VisitorDispatch`].
    ///
    /// Invokes [`VisitorDispatch::enter`] and [`VisitorDispatch::exit`] for
    /// every AST node encountered during traversal with the exception that if
    /// the `VisitorDispatch` returns
    /// [`VisitorControlFlow::Continue(Navigation::Skip)`], remaining children
    /// of the current node will be skipped.
    ///
    /// AST nodes from `sqlparser` are wrapped in a [`node::Node`]
    /// implementation and assigned a unique numeric ID so that derived metadata
    /// about nodes can be retained.
    fn accept<'ast, 'dispatch>(
        &'ast self,
        dispatch: &'dispatch mut dyn VisitorDispatch,
    ) -> EnterControlFlow
    where
        'ast: 'dispatch;
}
