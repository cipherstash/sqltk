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
use std::{error::Error, ops::ControlFlow};

#[derive(Clone, Copy)]
/// Used as the "continue" type in the [`ControlFlow`] value returned by all
/// visitors.
pub enum Nav {
    /// Skip visiting children of the current AST node
    Skip,
    /// Visit the children of the current AST node
    Visit,
}

/// [`ControlFlow`] type returned by [`Visitor::enter`].
pub type EnterControlFlow = ControlFlow<Box<dyn Error>, Nav>;

/// [`ControlFlow`] type returned by [`Visitor::exit`].
pub type ExitControlFlow = ControlFlow<Box<dyn Error>, ()>;

pub struct Enter;

impl Enter {
    pub fn visit() -> EnterControlFlow {
        EnterControlFlow::Continue(Nav::Visit)
    }

    pub fn skip() -> EnterControlFlow {
        EnterControlFlow::Continue(Nav::Skip)
    }

    pub fn error(error: Box<dyn Error>) -> EnterControlFlow {
        EnterControlFlow::Break(error)
    }
}

pub struct Exit;

impl Exit {
    pub fn normal() -> ExitControlFlow {
        ExitControlFlow::Continue(())
    }

    pub fn error(error: Box<dyn Error>) -> ExitControlFlow {
        ExitControlFlow::Break(error)
    }
}

/// Trait for types that visit a specific type of node.
#[allow(unused_variables)]
pub trait Visitor<'ast, Node: Visitable<'ast>> {
    /// Called when a node is entered.  The default implementation returns
    /// [`EnterControlFlow::Continue(Nav::Visit)`].
    fn enter(&mut self, node: &'ast Node) -> EnterControlFlow {
        Enter::visit()
    }

    /// Called when a node is exited.  The default implementation returns
    /// [`ExitControlFlow::Continue(())`].
    fn exit(&mut self, node: &'ast Node) -> ExitControlFlow {
        Exit::normal()
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
    /// [`EnterControlFlow::Continue(Nav::Skip)`], remaining children of the
    /// current node will be skipped.
    fn accept<VD: VisitorDispatch<'ast>>(&'ast self, dispatcher: &mut VD) -> EnterControlFlow;
}
