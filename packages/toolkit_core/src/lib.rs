mod ast_node_impls;
mod custom_display;
pub mod dispatch;
mod generated;
mod node;

// Re-export sqlparser
pub use sqlparser;
// Re-export bigdecimal
pub use bigdecimal;

pub use custom_display::*;
pub use dispatch::*;
pub use generated::concrete_node::*;
pub use node::*;
pub mod pipeline;

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
pub trait Visitor<'ast, T: AstNode<'ast>> {
    /// Called when a node is entered.
    ///
    /// The default implementation returns [`ControlFlow::Continue(Navigation::Visit)`].
    ///
    fn enter(&mut self, node: Node<'ast, T>) -> EnterControlFlow {
        ControlFlow::Continue(Navigation::Visit)
    }

    /// Called when a node is exited.  The default implementation returns
    /// [`ControlFlow::Continue(Navigation::Visit)`].  Note that the
    /// [`Navigation`] value returned in exit result is ignored.
    ///
    fn exit(&mut self, node: Node<'ast, T>) -> ExitControlFlow {
        ControlFlow::Continue(())
    }
}

/// Trait for types that can be visited by a [`VisitorDispatch`].
pub trait AstNode<'ast>
where
    Self: 'ast,
{
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
    fn accept<V>(&'ast self, visitor: &mut V) -> EnterControlFlow
    where
        V: VisitorDispatch<'ast>,
    {
        self.accept_and_identify(visitor, &mut NodeIdSequence::new())
    }

    /// Same as [`AstNode::accept`] but requires an additional `node_id_seq`
    /// parameter.
    ///
    /// *Not public API. Used by generated code.*
    #[doc(hidden)]
    fn accept_and_identify<V>(
        &'ast self,
        visitor: &mut V,
        node_id_seq: &mut NodeIdSequence,
    ) -> EnterControlFlow
    where
        V: VisitorDispatch<'ast>;
}