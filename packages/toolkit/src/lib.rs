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

mod ast_node_impls;
mod custom_display;
pub mod dispatch;
mod generated;
mod node;

// Re-export sqlparser
pub use sqlparser;

pub use custom_display::*;
pub use dispatch::*;
pub use generated::concrete_node::*;
pub use node::*;
pub mod pipeline;

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

/// [`ControlFlow`] type returned by [`Visitor::Enter`].
pub type EnterControlFlow = ControlFlow<(), Navigation>;

/// [`ControlFlow`] type returned by [`Visitor::Exit`].
pub type ExitControlFlow = ControlFlow<(), ()>;

/// Trait for types that visit one specific type of node.
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
    ) -> EnterControlFlow
    where
        V: VisitorDispatch<'ast>;
}

#[cfg(test)]
pub mod test {
    use std::ops::ControlFlow;

    use crate::{self as sqltk, ConcreteNode, ExitControlFlow};
    use sqlparser::ast;
    use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;
    use sqltk::{
        dispatch::Nope, AstNode, EnterControlFlow, Navigation, Node, Visitor,
        VisitorDispatch,
    };

    #[derive(VisitorDispatch)]
    pub struct Counter {
        pub count: usize,
    }

    impl Counter {
        pub fn new(count: usize) -> Self {
            Self { count }
        }
    }

    impl<'ast> Visitor<'ast, ast::Expr> for Counter {
        fn enter(&mut self, _: Node<'ast, ast::Expr>) -> EnterControlFlow {
            self.count += 1;
            ControlFlow::Continue(Navigation::Visit)
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
            fn enter(&mut self, node: Node<'ast, Option<ast::With>>) -> EnterControlFlow {
                self.items_enter.push(("Option<With>".into(), node.id()));
                ControlFlow::Continue(Navigation::Visit)
            }
        }
        impl<'ast> Visitor<'ast, Vec<ast::TableWithJoins>> for Recorder {
            fn enter(&mut self, node: Node<'ast, Vec<ast::TableWithJoins>>) -> EnterControlFlow {
                self.items_enter
                    .push(("Vec<TableWithJoins>".into(), node.id()));
                ControlFlow::Continue(Navigation::Visit)
            }
        }

        // Types that _should_ be visited because we know they'll be present
        // after parsing the `sql` expression below.
        impl<'ast> Visitor<'ast, Option<ast::Expr>> for Recorder {
            fn enter(&mut self, node: Node<'ast, Option<ast::Expr>>) -> EnterControlFlow {
                self.items_enter.push(("Option<Expr>".into(), node.id()));
                ControlFlow::Continue(Navigation::Visit)
            }
        }
        impl<'ast> Visitor<'ast, Vec<ast::SelectItem>> for Recorder {
            fn enter(&mut self, node: Node<'ast, Vec<ast::SelectItem>>) -> EnterControlFlow {
                self.items_enter.push(("Vec<SelectItem>".into(), node.id()));
                ControlFlow::Continue(Navigation::Visit)
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
            fn enter(&mut self, node: ConcreteNode<'ast>) -> EnterControlFlow {
                self.order_enter.push(node.to_string());
                ControlFlow::Continue(Navigation::Visit)
            }

            fn exit(&mut self, node: ConcreteNode<'ast>) -> ExitControlFlow {
                self.order_exit.push(node.to_string());
                ControlFlow::Continue(())
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

        assert_eq!(
            visitor.order_enter[0..12],
            [
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
            ]
        );
    }
}
