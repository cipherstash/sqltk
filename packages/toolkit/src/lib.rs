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

#[doc(inline)]
pub use sqltk_core::*;

#[doc(inline)]
pub use sqltk_derive::*;

#[cfg(test)]
pub mod test {
    use std::ops::ControlFlow;

    use crate::{self as sqltk};
    use sqltk::{
        dispatch::Nope, AstNode, EnterControlFlow, Navigation, Node, Visitor,
        pipeline::{self, InitializeError, ReadOnly, ReadWrite, RootScope, Scope, Stage},
        VisitorDispatch, SqlNode, ExitControlFlow, sqlparser::{self, ast, parser, dialect}
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
        let dialect = dialect::GenericDialect {};

        let sql = "SELECT a, b, 123, myfunc(b) \
                   FROM table_1 \
                   WHERE a > b AND b < 100 \
                   ORDER BY a DESC, b";

        let ast = parser::Parser::parse_sql(&dialect, sql).unwrap();

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

        let dialect = dialect::GenericDialect {};

        let sql = "SELECT 1 as a WHERE a > 0";

        let ast = parser::Parser::parse_sql(&dialect, sql).unwrap();

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
            fn enter(&mut self, node: SqlNode<'ast>) -> EnterControlFlow {
                self.order_enter.push(node.to_string());
                ControlFlow::Continue(Navigation::Visit)
            }

            fn exit(&mut self, node: SqlNode<'ast>) -> ExitControlFlow {
                self.order_exit.push(node.to_string());
                ControlFlow::Continue(())
            }
        }

        let dialect = dialect::GenericDialect {};

        let sql = "SELECT a, b, 123, myfunc(b) \
                   FROM table_1 \
                   WHERE a > b AND b < 100 \
                   ORDER BY a DESC, b";

        let ast = parser::Parser::parse_sql(&dialect, sql).unwrap();

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

    #[test]
    fn basic_pipeline() {
        #[derive(Debug, Eq, PartialEq)]
        struct ExprsBalanced(bool);

        #[derive(VisitorDispatch)]
        struct BalancedExprsCheck {
            expr_enter_count: ReadOnly<ExprEnterCount>,
            expr_exit_count: ReadOnly<ExprExitCount>,
            exprs_balanced: ReadWrite<ExprsBalanced>,
        }

        impl<'ast, 'scope> Stage<'ast, 'scope> for BalancedExprsCheck {
            fn init_exit(scope: &mut impl Scope<'scope>) -> Result<Self, InitializeError> {
                scope
                    .import::<ExprEnterCount>()
                    .import::<ExprExitCount>()
                    .export(ExprsBalanced(true))
                    .resolve()
                    .map(|(expr_enter_count, expr_exit_count, exprs_balanced)| Self {
                        expr_enter_count,
                        expr_exit_count,
                        exprs_balanced,
                    })
                    .map_err(|_| InitializeError)
            }
        }

        impl<'ast> Visitor<'ast, ast::Expr> for BalancedExprsCheck {
            fn enter(&mut self, _: Node<'ast, ast::Expr>) -> EnterControlFlow {
                self.exprs_balanced.get_mut().0 = false;
                ControlFlow::Continue(Navigation::Visit)
            }

            fn exit(&mut self, _: Node<'ast, ast::Expr>) -> ExitControlFlow {
                self.exprs_balanced.get_mut().0 =
                    self.expr_enter_count.get().0 == self.expr_exit_count.get().0;
                ControlFlow::Continue(())
            }
        }

        struct ExprEnterCount(usize);
        struct ExprExitCount(usize);

        #[derive(VisitorDispatch)]
        struct ExprCounter {
            expr_enter_count: ReadWrite<ExprEnterCount>,
            expr_exit_count: ReadWrite<ExprExitCount>,
        }

        impl<'ast, 'scope> Stage<'ast, 'scope> for ExprCounter {
            fn init_enter(scope: &mut impl Scope<'scope>) -> Result<(), InitializeError> {
                scope.export(ExprEnterCount(0)).export(ExprExitCount(0));

                Ok(())
            }

            fn init_exit(scope: &mut impl Scope<'scope>) -> Result<Self, InitializeError> {
                scope
                    .import_owned::<ExprEnterCount>()
                    .import_owned::<ExprExitCount>()
                    .resolve()
                    .map(|(expr_enter_count, expr_exit_count)| Self {
                        expr_enter_count,
                        expr_exit_count,
                    })
                    .map_err(|_| InitializeError)
            }
        }

        impl<'ast> Visitor<'ast, ast::Expr> for ExprCounter {
            fn enter(&mut self, _: Node<'ast, ast::Expr>) -> EnterControlFlow {
                self.expr_enter_count.get_mut().0 += 1;
                ControlFlow::Continue(Navigation::Visit)
            }

            fn exit(&mut self, _: Node<'ast, ast::Expr>) -> ExitControlFlow {
                self.expr_exit_count.get_mut().0 += 1;
                ControlFlow::Continue(())
            }
        }

        if let Ok(mut pipeline) =
            pipeline::build::<(BalancedExprsCheck, ExprCounter)>(RootScope::default())
        {
            let dialect = dialect::GenericDialect {};

            let sql = "SELECT a, b, 123, myfunc(b) \
                       FROM table_1 \
                       WHERE a > b AND b < 100 \
                       ORDER BY a DESC, b";

            let ast = parser::Parser::parse_sql(&dialect, sql).unwrap();

            ast.accept(&mut pipeline);

            if let Ok(expr_balanced) = pipeline.get::<ExprsBalanced>() {
                assert_eq!(*expr_balanced.get(), ExprsBalanced(true));
            } else {
                assert!(false, "Could not read result from scope")
            }
        } else {
            assert!(false, "Pipeline construction failed")
        };
    }

    #[test]
    fn basic_edit_pipeline() {
        // First VisitorDispatch in the pipeline must build a shadow AST of Node<'ast, T: AstNode>
        // (need a SqlNode::wrapped(&self) method)
        // The Scope of the analysis Pipeline will contain the shadow AST + analysis results
    }
}