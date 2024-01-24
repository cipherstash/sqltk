//!
//! This crate implements an enhanced Visitor implementation suitable for
//! semantic analysis of SQL.
//!
//! The AST is provided by the `sqlparser` crate - which this crate re-exports.
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
//! $ cargo install cargo-expand
//! $ cargo add sqltk
//! ```
//!
//! If your crate makes use of `sqlparser`'s `bigdecimal` feature then add
//! `sqltk` with the `bigdecimal` feature:
//!
//! ```shell
//! $ cargo add sqltk --features bigdecimal
//! ```

mod access;
mod ast_node_impls;
mod compose;
mod concrete_node;
mod custom_display;
mod dispatch;
mod node;
mod pipeline;

// Re-export sqlparser
pub use sqlparser;

pub use access::*;
pub use concrete_node::*;
pub use custom_display::*;
pub use dispatch::*;
pub use node::*;

/// Exposed to allow macro-generated code and some internals to work.
#[doc(hidden)]
pub mod private;

pub use sqltk_derive::*;

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
    fn enter(&mut self, node: Node<'ast, T>) -> VisitorControlFlow {
        nav_visit()
    }

    /// Called when a node is exited.  The default implementation returns
    /// [`ControlFlow::Continue(Navigation::Visit)`].  Note that the
    /// [`Navigation`] value returned in exit result is ignored.
    ///
    fn exit(&mut self, node: Node<'ast, T>) -> VisitorControlFlow {
        nav_visit()
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
    fn accept<V>(&'ast self, visitor: &mut V) -> VisitorControlFlow
    where
        V: VisitorDispatch<'ast>,
    {
        self.accept_with_node_builder(visitor, &mut NodeBuilder::new())
    }

    /// Same as [`AstNode::accept`] but requires an additional `node_builder`
    /// parameter.
    ///
    /// *Not public API. Used by generated code.*
    #[doc(hidden)]
    fn accept_with_node_builder<V>(
        &'ast self,
        visitor: &mut V,
        node_builder: &mut NodeBuilder,
    ) -> VisitorControlFlow
    where
        V: VisitorDispatch<'ast>;
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
    use crate::{self as sqltk, ConcreteNode};
    use sqlparser::ast;
    use sqlparser::ast::Expr;
    use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;
    use sqltk::{nav_visit, AstNode, Node, Visitor, VisitorControlFlow, VisitorDispatch};

    #[derive(VisitorDispatch)]
    pub struct Counter {
        pub count: usize,
    }

    impl Counter {
        pub fn new(count: usize) -> Self {
            Self { count }
        }
    }

    impl<'ast> Visitor<'ast, Expr> for Counter {
        fn enter(&mut self, _: Node<'ast, Expr>) -> VisitorControlFlow {
            self.count += 1;
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

        let mut visitor = Counter::new(0);

        let _result = ast.accept(&mut visitor);

        // The expressions are:
        // In the SELECT projection: a, b, 123, myfunc(6), 6
        // In the WHERE clause: a, b, a > b, b, 100, b < 100, a > b AND b < 100
        // In the ORDER BY clause: a, b
        // Total: 14
        assert_eq!(14usize, visitor.count);
    }

    #[test]
    fn fallback() {
        let ast = String::from("OH HAI!");

        let mut visitor = Counter::new(0);

        let _result = ast.accept(&mut visitor);

        assert_eq!(visitor.count, 0usize);
    }

    #[test]
    fn visit_useless() {
        #[derive(VisitorDispatch, Default)]
        pub struct Recorder {
            pub items_enter: Vec<(String, usize)>,
        }

        // Types that should _not_ be visited because we know it'll be
        // None/empty with the `sql` expression below.
        impl<'ast> Visitor<'ast, Option<ast::With>> for Recorder {
            fn enter(&mut self, node: Node<'ast, Option<ast::With>>) -> VisitorControlFlow {
                self.items_enter.push(("Option<With>".into(), node.id()));
                nav_visit()
            }
        }
        impl<'ast> Visitor<'ast, Vec<ast::TableWithJoins>> for Recorder {
            fn enter(&mut self, node: Node<'ast, Vec<ast::TableWithJoins>>) -> VisitorControlFlow {
                self.items_enter
                    .push(("Vec<TableWithJoins>".into(), node.id()));
                nav_visit()
            }
        }

        // Types that _should_ be visited because we know they'll be present
        // after parsing the `sql` expression below.
        impl<'ast> Visitor<'ast, Option<ast::Expr>> for Recorder {
            fn enter(&mut self, node: Node<'ast, Option<ast::Expr>>) -> VisitorControlFlow {
                self.items_enter.push(("Option<Expr>".into(), node.id()));
                nav_visit()
            }
        }
        impl<'ast> Visitor<'ast, Vec<ast::SelectItem>> for Recorder {
            fn enter(&mut self, node: Node<'ast, Vec<ast::SelectItem>>) -> VisitorControlFlow {
                self.items_enter.push(("Vec<SelectItem>".into(), node.id()));
                nav_visit()
            }
        }

        let dialect = GenericDialect {};

        let sql = "SELECT 1 as a WHERE a > 0";

        let ast = Parser::parse_sql(&dialect, sql).unwrap();

        let mut visitor = Recorder::default();

        ast.accept(&mut visitor);

        let mut expected_items = Vec::<(String, usize)>::new();
        expected_items.push(("Option<Expr>".to_string(), 17usize));
        expected_items.push(("Vec<SelectItem>".to_string(), 8usize));

        let mut visitor_items = visitor.items_enter;
        visitor_items.sort();

        assert_eq!(visitor_items, expected_items);
    }

    #[test]
    fn source_node_reachable_fields_are_visited_first() {
        #[derive(Default)]
        struct Recorder {
            pub order_enter: Vec<String>,
            pub order_exit: Vec<String>,
        }

        impl<'ast> VisitorDispatch<'ast> for Recorder {
            fn dispatch_enter(&mut self, concrete_node: ConcreteNode<'ast>) -> VisitorControlFlow {
                self.order_enter.push(concrete_node.to_string());
                nav_visit()
            }

            fn dispatch_exit(&mut self, concrete_node: ConcreteNode<'ast>) -> VisitorControlFlow {
                self.order_exit.push(concrete_node.to_string());
                nav_visit()
            }
        }

        let dialect = GenericDialect {};

        let sql = "SELECT a, b, 123, myfunc(b) \
                   FROM table_1 \
                   WHERE a > b AND b < 100 \
                   ORDER BY a DESC, b";

        let ast = Parser::parse_sql(&dialect, sql).unwrap();

        let mut visitor = Recorder::default();

        ast.accept(&mut visitor);

        assert_eq!(visitor.order_enter[0..12], [
            "Vec<Statement> (ID: 0)",
            "Statement (ID: 1)",
            "Box<Query> (ID: 2)",
            "Query (ID: 3)",
            "Box<SetExpr> (ID: 4)",
            "SetExpr (ID: 5)",
            "Box<Select> (ID: 6)",
            "Select (ID: 7)",
            "Vec<TableWithJoins> (ID: 8)",
            "TableWithJoins (ID: 9)",
            "TableFactor (ID: 10)",
            "ObjectName (ID: 11)",
        ]);
    }
}
