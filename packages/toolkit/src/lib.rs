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
    use std::{any::TypeId, cell::RefCell, collections::HashSet, ops::ControlFlow, rc::Rc};

    use derive_more::Constructor;

    use crate::{self as sqltk};
    use sqltk::{
        dispatch::Nope,
        sqlparser::{self, ast, dialect, parser},
        EnterControlFlow, ExitControlFlow, Navigation, SqlNode, Visitable, Visitor,
        VisitorDispatch,
    };

    use sqltk_core::{Pipeline, PipelineBuilder, ReadOnly, ReadWrite};

    #[derive(VisitorDispatch)]
    pub struct Counter {
        pub count: usize,
    }

    impl<'ast> Counter {
        pub fn new(count: usize) -> Self {
            Self { count }
        }
    }

    impl<'ast> Visitor<'ast, ast::Expr> for Counter {
        fn enter(&mut self, _: &'ast ast::Expr) -> EnterControlFlow {
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
            pub items_enter: Vec<String>,
        }

        // Types that should _not_ be visited because we know it'll be
        // None/empty with the `sql` expression below.
        impl<'ast> Visitor<'ast, Option<ast::With>> for Recorder {
            fn enter(&mut self, _: &'ast Option<ast::With>) -> EnterControlFlow {
                self.items_enter.push("Option<With>".into());
                ControlFlow::Continue(Navigation::Visit)
            }
        }

        impl<'ast> Visitor<'ast, Vec<ast::TableWithJoins>> for Recorder {
            fn enter(&mut self, _: &'ast Vec<ast::TableWithJoins>) -> EnterControlFlow {
                self.items_enter.push("Vec<TableWithJoins>".into());
                ControlFlow::Continue(Navigation::Visit)
            }
        }

        // Types that _should_ be visited because we know they'll be present
        // after parsing the `sql` expression below.
        impl<'ast> Visitor<'ast, Option<ast::Expr>> for Recorder {
            fn enter(&mut self, _: &'ast Option<ast::Expr>) -> EnterControlFlow {
                self.items_enter.push("Option<Expr>".into());
                ControlFlow::Continue(Navigation::Visit)
            }
        }

        impl<'ast> Visitor<'ast, Vec<ast::SelectItem>> for Recorder {
            fn enter(&mut self, _: &'ast Vec<ast::SelectItem>) -> EnterControlFlow {
                self.items_enter.push("Vec<SelectItem>".into());
                ControlFlow::Continue(Navigation::Visit)
            }
        }

        let dialect = dialect::GenericDialect {};

        let sql = "SELECT 1 as a WHERE a > 0";

        let ast = parser::Parser::parse_sql(&dialect, sql).unwrap();

        let mut visitor = Recorder::default();

        ast.accept(&mut visitor);

        let mut expected_items = Vec::<String>::new();
        expected_items.push("Option<Expr>".to_string());
        expected_items.push("Vec<SelectItem>".to_string());

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
                "Vec<Statement>",
                "Statement",
                "Box<Query>",
                "Query",
                "Box<SetExpr>",
                "SetExpr",
                "Box<Select>",
                "Select",
                "Vec<TableWithJoins>",
                "TableWithJoins",
                "TableFactor",
                "ObjectName",
            ]
        );
    }

    #[test]
    fn basic_pipeline() {
        #[derive(Debug, Eq, PartialEq)]
        struct ExprsBalanced(bool);

        #[derive(Debug, Eq, PartialEq)]
        struct ExprEnterCount(usize);

        #[derive(Debug, Eq, PartialEq)]
        struct ExprExitCount(usize);

        #[derive(VisitorDispatch, Constructor)]
        struct BalancedExprsCheck {
            expr_enter_count: ReadOnly<ExprEnterCount>,
            expr_exit_count: ReadOnly<ExprExitCount>,
            exprs_balanced: ReadWrite<ExprsBalanced>,
        }

        #[derive(VisitorDispatch, Constructor)]
        struct ExprCounter {
            expr_enter_count: ReadWrite<ExprEnterCount>,
            expr_exit_count: ReadWrite<ExprExitCount>,
        }

        let expr_enter_count = &Rc::new(RefCell::new(ExprEnterCount(0)));
        let expr_exit_count = &Rc::new(RefCell::new(ExprExitCount(0)));
        let exprs_balanced = &Rc::new(RefCell::new(ExprsBalanced(true)));

        let pipeline = PipelineBuilder::new()
            .add_stage(BalancedExprsCheck::new(
                expr_enter_count.into(),
                expr_exit_count.into(),
                exprs_balanced.into(),
            ))
            .add_stage(ExprCounter::new(
                expr_enter_count.into(),
                expr_exit_count.into(),
            ))
            .output::<ReadOnly<ExprsBalanced>>(exprs_balanced.into())
            .build();

        impl<'ast> Visitor<'ast, ast::Expr> for BalancedExprsCheck {
            fn enter(&mut self, _: &'ast ast::Expr) -> EnterControlFlow {
                self.exprs_balanced.get_mut().0 = false;
                ControlFlow::Continue(Navigation::Visit)
            }

            fn exit(&mut self, _: &'ast ast::Expr) -> ExitControlFlow {
                self.exprs_balanced.get_mut().0 =
                    self.expr_enter_count.get().0 == self.expr_exit_count.get().0;
                ControlFlow::Continue(())
            }
        }

        impl<'ast> Visitor<'ast, ast::Expr> for ExprCounter {
            fn enter(&mut self, _: &'ast ast::Expr) -> EnterControlFlow {
                self.expr_enter_count.get_mut().0 += 1;
                ControlFlow::Continue(Navigation::Visit)
            }

            fn exit(&mut self, _: &'ast ast::Expr) -> ExitControlFlow {
                self.expr_exit_count.get_mut().0 += 1;
                ControlFlow::Continue(())
            }
        }

        let dialect = dialect::GenericDialect {};

        let sql = "SELECT a, b, 123, myfunc(b) \
                    FROM table_1 \
                    WHERE a > b AND b < 100 \
                    ORDER BY a DESC, b";

        let ast = parser::Parser::parse_sql(&dialect, sql).unwrap();

        match pipeline.execute(&ast) {
            Ok(exprs_balanced) => {
                assert_eq!(*exprs_balanced.get(), ExprsBalanced(true));
            }
            Err(_) => assert!(false),
        }
    }

    // This test is a sanity check (not thorough at all - it only tests a small
    // fragment of grammar) that sqlparser does not reuse nodes in the AST,
    // as-in two parents point to the same child. Notionally, in memory that
    // would be OK because it would not be observable by anything traversing the
    // AST: they would seem like seperate nodes.
    //
    // The only time this would be obvservable would be when casting a reference
    // to a usize and checking if the addresses of two nodes with different
    // parents and the addresses of the "different" nodes happen to be the same.
    //
    // The reason for this test: I'm thinking of ditching the Node abstraction.
    // It exists in order to assign a unique ID to a sqlparser AST type so that
    // metadata can be attached.  The problem is that the IDs are assigned at
    // AST traversal time.
    //
    // The implication is that if multiple passes are required and traversal
    // order changes, the IDs will no longer line up. More realistically, if
    // some nodes are removed in an edit pass, IDs will definitely no longer
    // line up and this does not require a change in traversal order.
    //
    // What I'm considering building is a AstMetadata type that stores metadata
    // using a (node address, TypeId) tuple as a key. TypeId is needed in
    // addition to the address: there are a number of situations where values of
    // two different types share an address, e.g. the address of a struct and
    // the address of its first field will share the same memory address.
    //
    // Relying on memory addresses being fixed is generally regarded as a Bad
    // Thing in Rust, because in Rust values will move when they change
    // ownership and therefore any stored addresses will become invalid.
    //
    // But if we can prevent an AST from being moved it is perfectly safe. We
    // can prevent an AST from being moved by ensuring that the AstMetaData type
    // takes and retains a reference to the root node of the AST. The borrow
    // checker will prevent the AST from being moved until the AstMetaData type
    // is dropped.
    #[test]
    fn sqlparser_does_not_reuse_ast_nodes() {
        let dialect = dialect::GenericDialect {};

        let sql = "SELECT a, b, 123, myfunc(b) \
                    FROM table_1 \
                    WHERE a > b AND b < 100 \
                    ORDER BY a DESC, b";

        let ast = parser::Parser::parse_sql(&dialect, sql).unwrap();

        #[derive(VisitorDispatch, Default)]
        struct AddrChecker {
            count: usize,
            node_addrs: HashSet<(TypeId, usize)>,
        }

        impl<'ast, T: Visitable<'ast> + 'static> Visitor<'ast, T> for AddrChecker {
            fn enter(&mut self, node: &'ast T) -> EnterControlFlow {
                self.count += 1;
                self.node_addrs
                    .insert((TypeId::of::<T>(), node as *const T as usize));
                ControlFlow::Continue(Navigation::Visit)
            }
        }

        let mut addr_checker = AddrChecker::default();

        ast.accept(&mut addr_checker);

        assert_eq!(addr_checker.count, addr_checker.node_addrs.len());
    }

    #[test]
    fn basic_edit_pipeline() {
        // First VisitorDispatch in the pipeline must build a shadow AST of Node<'ast, T: Visitable>
        // (need a SqlNode::wrapped(&self) method)
        // The Scope of the analysis Pipeline will contain the shadow AST + analysis results
    }
}
