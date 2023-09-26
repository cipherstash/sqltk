//!
//! This crate implements an enhanced Visitor implementation suitable for
//! semantic analysis of SQL.
//!
//! ## Key features
//!
//! 1. Full coverage of all AST node types from `sqlparser` (including all field
//!    types and container types (`Vec`, `Option` & `Box`)) and primitives.
//!
//! 2. Includes a facility for generating unique identifers for all AST node types.
//!
//! 3. A facility for associating user-defined metadata with AST nodes during
//!    traversal.
//!
//! 4. An API for creating pipelines composed from multiple Visitors.
//!
//! ## Installation
//!
//! ```shell
//! $ cargo add sqlparser_ast_toolkit
//! ```
//!
//! If your crate makes use of `sqlparser`'s `bigdecimal` feature then add
//! `sqlparser_ast_toolkit` with the `bigdecimal` feature:
//!
//! ```shell
//! $ cargo add sqlparser_ast_toolkit --features bigdecimal
//! ```

mod access;
mod ast_node_impls;
mod compose;
mod dispatch;
mod node;
mod pipeline;

// Re-export sqlparser
pub use sqlparser;

pub use access::*;
pub use dispatch::*;
pub use node::*;

/// Exposed to allow macro-generated code and some internals to work.
#[doc(hidden)]
pub mod private;

pub use sqlparser_ast_toolkit_derive::*;

use std::ops::ControlFlow;
use private::visit;

#[derive(Clone, Copy)]
/// Used as the "continue" type in the [`ControlFlow`] value returned by all
/// visitors.
pub enum Navigation {
    /// Skip visiting children of the current AST node
    Skip,
    /// Visit the children of the current AST node
    Visit,
}

/// Mandatory [`ControlFlow`] type used throughout this crate.
/// Convenient type alias for [`Visitor`] implementors.
pub type VisitorControlFlow = ControlFlow<(), Navigation>;

/// Trait for types that visit one specific type of node.
#[allow(unused_variables)]
pub trait Visitor<'ast, T: 'ast + AstNode<'ast>> {
    /// Called when a node is entered.
    ///
    /// The default implementation returns [`ControlFlow::Continue(Navigation::Visit)`].
    ///
    fn enter(&self, node: Node<'ast, T>) -> VisitorControlFlow {
        nav_visit()
    }

    /// Called when a node is exited.  The default implementation returns
    /// [`ControlFlow::Continue(Navigation::Visit)`].  Note that the
    /// [`Navigation`] value returned in exit result is ignored.
    ///
    fn exit(&self, node: Node<'ast, T>) -> VisitorControlFlow {
        nav_visit()
    }
}

/// Trait for types that can be visited by a [`VisitorDispatch`].
pub trait AstNode<'ast>
where
    Self: 'ast,
    // Node<'ast, Self>: Into<ConcreteNode<'ast>>,
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
    fn accept<V>(&'ast self, visitor: &V) -> VisitorControlFlow where V: VisitorDispatch<'ast> {
        self.accept_with_id_iter(visitor, &mut NodeBuilder::new())
    }

    /// Same as [`Visitable::accept`] but requires an additional `node_builder`
    /// parameter.
    ///
    /// *Not public API. Used by generated code.*
    #[doc(hidden)]
    fn accept_with_id_iter<V>(
        &'ast self,
        visitor: &V,
        node_builder: &mut NodeBuilder,
    ) -> VisitorControlFlow where V: VisitorDispatch<'ast>;
}


/// Convenience function to return a value that indicates traversal of the AST
/// should not proceed to children of the current node.
///
pub fn nav_skip() -> VisitorControlFlow {
    ControlFlow::Continue(Navigation::Skip)
}

/// Convenience function to return a value that indicates traversal of the AST
/// should continue with children of the current node.
///
pub fn nav_visit() -> VisitorControlFlow {
    ControlFlow::Continue(Navigation::Visit)
}

/// Convenience function abort traversal of the AST. This will bubble all of way
/// up the stack to the top-most [`Node::accept`] invocation.
///
pub fn nav_break() -> VisitorControlFlow {
    ControlFlow::Break(())
}

#[cfg(test)]
pub mod test {
    use crate::{self as sqlparser_ast_toolkit};
    use sqlparser::ast::Expr;
    use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;
    use sqlparser_ast_toolkit::{
        nav_visit, Node, AstNode, Visitor, VisitorControlFlow, VisitorDispatch,
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(VisitorDispatch)]
    pub struct Counter {
        count: Rc<RefCell<usize>>,
    }

    impl Counter {
        pub fn new(count: usize) -> Self {
            Self {
                count: Rc::new(RefCell::new(count)),
            }
        }

        pub fn count(&self) -> usize {
          *self.count.borrow()
        }
    }

    impl<'ast> Visitor<'ast, Expr> for Counter {
        fn enter(&self, _: Node<'ast, Expr>) -> VisitorControlFlow {
            *self.count.borrow_mut() += 1;
            nav_visit()
        }
    }

    #[test]
    fn basic() {
        let dialect = GenericDialect {};

        let sql = "SELECT a, b, 123, myfunc(b) \
                   FROM table_1 \
                   WHERE a > b AND b < 100 \
                   ORDER BY a DESC, b";

        let ast = Parser::parse_sql(&dialect, sql).unwrap();

        let visitor = Counter::new(0);

        let _result = ast.accept(&visitor);

        // The expressions are:
        // In the SELECT projection: a, b, 123, myfunc(6), 6
        // In the WHERE clause: a, b, a > b, b, 100, b < 100, a > b AND b < 100
        // In the ORDER BY clause: a, b
        // Total: 14
        assert_eq!(14usize, visitor.count());
    }

    #[test]
    fn fallback() {
        let ast = String::from("OH HAI!");

        let visitor = Counter::new(0);

        let _result = ast.accept(&visitor);

        assert_eq!(visitor.count(), 0usize);
    }
}
