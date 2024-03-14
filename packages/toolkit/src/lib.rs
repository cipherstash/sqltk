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
        sqlparser::{ast, dialect, parser},
        Flow, Pipeline, SqlNode, Visitable, Visitor, VisitorControlFlow, VisitorDispatch,
    };
    use sqltk_core::{Break, PipelineError};

    #[derive(VisitorDispatch)]
    #[visitor_dispatch(state = usize)]
    pub struct Counter;

    impl<'ast> Visitor<'ast, ast::Expr, usize> for Counter {
        fn enter(&self, _: &'ast ast::Expr, state: usize) -> VisitorControlFlow<usize> {
            Flow::cont(state + 1)
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

        let visitor = Counter;
        let counter: usize = 0;

        // The expressions are:
        // In the SELECT projection: a, b, 123, myfunc(6), 6
        // In the WHERE clause: a, b, a > b, b, 100, b < 100, a > b AND b < 100
        // In the ORDER BY clause: a, b
        // Total: 14
        assert_eq!(
            VisitorControlFlow::Continue(14),
            ast.accept(&visitor, counter)
        );
    }

    #[test]
    fn fallback() {
        let ast = String::from("OH HAI!");

        let visitor = Counter;

        assert_eq!(VisitorControlFlow::Continue(0), ast.accept(&visitor, 0));
    }

    #[test]
    fn visit_useless() {
        #[derive(VisitorDispatch)]
        #[visitor_dispatch(state = Vec<String>)]
        pub struct Recorder;

        // Types that should _not_ be visited because we know it'll be
        // None/empty with the `sql` expression below.
        impl<'ast> Visitor<'ast, Option<ast::With>, Vec<String>> for Recorder {
            fn enter(
                &self,
                _: &'ast Option<ast::With>,
                mut state: Vec<String>,
            ) -> VisitorControlFlow<Vec<String>> {
                state.push("Option<With>".into());
                Flow::cont(state)
            }
        }

        impl<'ast> Visitor<'ast, Vec<ast::TableWithJoins>, Vec<String>> for Recorder {
            fn enter(
                &self,
                _: &'ast Vec<ast::TableWithJoins>,
                mut state: Vec<String>,
            ) -> VisitorControlFlow<Vec<String>> {
                state.push("Vec<TableWithJoins>".into());
                Flow::cont(state)
            }
        }

        // Types that _should_ be visited because we know they'll be present
        // after parsing the `sql` expression below.
        impl<'ast> Visitor<'ast, Option<ast::Expr>, Vec<String>> for Recorder {
            fn enter(
                &self,
                _: &'ast Option<ast::Expr>,
                mut state: Vec<String>,
            ) -> VisitorControlFlow<Vec<String>> {
                state.push("Option<Expr>".into());
                Flow::cont(state)
            }
        }

        impl<'ast> Visitor<'ast, Vec<ast::SelectItem>, Vec<String>> for Recorder {
            fn enter(
                &self,
                _: &'ast Vec<ast::SelectItem>,
                mut state: Vec<String>,
            ) -> VisitorControlFlow<Vec<String>> {
                state.push("Vec<SelectItem>".into());
                Flow::cont(state)
            }
        }

        let dialect = dialect::GenericDialect {};

        let sql = "SELECT 1 as a WHERE a > 0";

        let ast = parser::Parser::parse_sql(&dialect, sql).unwrap();

        let visitor = Recorder;

        match ast.accept(&visitor, Vec::<String>::new()) {
            ControlFlow::Continue(mut result) => {
                result.sort();
                let mut expected_items = Vec::<String>::new();
                expected_items.push("Option<Expr>".to_string());
                expected_items.push("Vec<SelectItem>".to_string());
                assert_eq!(result, expected_items)
            }
            ControlFlow::Break(_) => assert!(false),
        }
    }

    #[test]
    fn source_node_reachable_fields_are_visited_first() {
        #[derive(Default)]
        struct RecorderContext {
            order_enter: Vec<String>,
            order_exit: Vec<String>,
        }

        struct Recorder;

        impl<'ast> VisitorDispatch<'ast, RecorderContext> for Recorder {
            fn enter(
                &self,
                node: SqlNode<'ast>,
                mut state: RecorderContext,
            ) -> VisitorControlFlow<RecorderContext> {
                state.order_enter.push(node.to_string());
                Flow::cont(state)
            }

            fn exit(
                &self,
                node: SqlNode<'ast>,
                mut state: RecorderContext,
            ) -> VisitorControlFlow<RecorderContext> {
                state.order_exit.push(node.to_string());
                Flow::cont(state)
            }
        }

        let dialect = dialect::GenericDialect {};

        let sql = "SELECT a, b, 123, myfunc(b) \
                   FROM table_1 \
                   WHERE a > b AND b < 100 \
                   ORDER BY a DESC, b";

        let ast = parser::Parser::parse_sql(&dialect, sql).unwrap();

        let visitor = Recorder;
        let state = RecorderContext::default();

        let result = ast.accept(&visitor, state);

        match result {
            VisitorControlFlow::Continue(RecorderContext { order_enter, .. }) => {
                assert_eq!(
                    order_enter[0..12],
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
            _ => assert!(false),
        }
    }

    #[test]
    fn basic_pipeline() {
        #[derive(Default)]
        struct MyState {
            enter_count: usize,
            exit_count: usize,
            balanced: bool,
        }

        impl MyState {
            fn inc_enter_count(mut self) -> Self {
                self.enter_count += 1;
                self
            }

            fn inc_exit_count(mut self) -> Self {
                self.exit_count += 1;
                self
            }

            fn update_balanced(mut self) -> Self {
                self.balanced = self.enter_count == self.exit_count;
                self
            }
        }

        impl From<MyState> for bool {
            fn from(value: MyState) -> Self {
                value.balanced
            }
        }

        #[derive(VisitorDispatch)]
        #[visitor_dispatch(state = MyState)]
        struct BalancedExprsCheck;

        #[derive(VisitorDispatch)]
        #[visitor_dispatch(state = MyState)]
        struct ExprCounter;

        impl<'ast> Visitor<'ast, ast::Expr, MyState> for BalancedExprsCheck {
            fn exit(
                &self,
                _: &'ast ast::Expr,
                state: MyState,
            ) -> VisitorControlFlow<MyState> {
                Flow::cont(state.update_balanced())
            }
        }

        impl<'ast> Visitor<'ast, ast::Expr, MyState> for ExprCounter {
            fn enter(
                &self,
                _: &'ast ast::Expr,
                state: MyState,
            ) -> VisitorControlFlow<MyState> {
                Flow::cont(state.inc_enter_count())
            }

            fn exit(
                &self,
                _: &'ast ast::Expr,
                state: MyState,
            ) -> VisitorControlFlow<MyState> {
                Flow::cont(state.inc_exit_count())
            }
        }

        let dialect = dialect::GenericDialect {};

        let sql = "SELECT a, b, 123, myfunc(b) \
                    FROM table_1 \
                    WHERE a > b AND b < 100 \
                    ORDER BY a DESC, b";

        let ast = parser::Parser::parse_sql(&dialect, sql).unwrap();

        struct MyPipeline;

        impl<'out, 'ast: 'out> Pipeline<'out, 'ast, MyState> for MyPipeline {
            type Output = bool;

            fn execute<N>(&self, node: &'ast N) -> Result<Self::Output, PipelineError<MyState>>
            where
                N: Visitable<'ast>,
                &'ast N: Into<SqlNode<'ast>>,
            {
                let state = MyState::default();

                let stages: &[Box<dyn VisitorDispatch<'ast, MyState>>] =
                    &[Box::new(BalancedExprsCheck), Box::new(ExprCounter)];

                match node.accept(&stages, state) {
                    ControlFlow::Continue(state) => Ok(state.into()),
                    ControlFlow::Break(Break::Err(err, state)) => {
                        Err(PipelineError::Error(err, state))
                    }
                    ControlFlow::Break(_) => unimplemented!("Decide how to handle this"),
                }
            }
        }

        let pipeline = MyPipeline;

        match pipeline.execute(&ast) {
            Ok(exprs_balanced) => assert!(exprs_balanced),
            Err(_) => assert!(false),
        }
    }
}
