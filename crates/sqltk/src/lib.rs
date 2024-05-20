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
//! 2. Support for annotating AST nodes with user-defined metadata during
//!    traversal (see: [`sqltk-analysis`])
//!
//! 4. An API for composing [`Visitor`] implementations to facilitate complex
//!    SQL analysis scenarios made from small, unit testable pieces
//!    (see: [`sqltk-analysis`]).
//!
//! ## Installation
//!
//! ```shell
//! $ cargo install cargo-expand
//! $ cargo add sqltk
//! # optional:
//! $ cargo add sqltk-analysis
//! ```
//!
//! ## Example
//!
//! Count the number of Expr nodes in an AST.
//!
//! ```
//! use sqltk::prelude::*;
//!
//! let dialect = GenericDialect {};
//!
//! let sql = "SELECT a, b, 123, myfunc(b) \
//!            FROM table_1 \
//!            WHERE a > b AND b < 100 \
//!            ORDER BY a DESC, b";
//!
//! let ast = Parser::parse_sql(&dialect, sql).unwrap();
//!
//! #[derive(Visitor)]
//! #[visitor(
//!     state_ty = usize,
//!     enter = {
//!         if node.is::<Expr>() {
//!             flow::cont(state + 1)
//!         } else {
//!             flow::cont(state)
//!         }
//!     }
//! )]
//! struct CountOfExprNodes;
//!
//! // The expressions are:
//! //   In the SELECT projection: a, b, 123, myfunc(b), b
//! //   In the WHERE clause: a, b, a > b, b, 100, b < 100, a > b AND b < 100
//! //   In the ORDER BY clause: a, b
//! // Total: 14
//!
//! match ast.evaluate(&CountOfExprNodes, 0) {
//!     Ok(count) => assert_eq!(14, count),
//!     err => panic!("{:#?}", err)
//! };
//! ```

// This module re-exports sqltk-core & sqltk-derive, and is where the
// crate-level tests live.
//
// No functionality should be created here (beyond simply re-exporting).

#[doc(inline)]
pub use sqltk_core::*;

#[doc(inline)]
pub use sqltk_derive::*;

pub mod prelude;

#[cfg(test)]
mod test {
    use std::fmt::Debug;

    use sqlparser::{ast::{Expr, SelectItem, TableWithJoins, With}, dialect, parser};
    use sqltk_core::{flow, Visitable};
    use sqltk_derive::Visitor;

    #[test]
    fn basic() {
        let dialect = dialect::GenericDialect {};

        let sql = "SELECT a, b, 123, myfunc(b) \
                   FROM table_1 \
                   WHERE a > b AND b < 100 \
                   ORDER BY a DESC, b";

        let ast = parser::Parser::parse_sql(&dialect, sql).unwrap();

        #[derive(Debug, Visitor)]
        #[visitor(
            state_ty = usize,
            enter = {
                if node.is::<Expr>() {
                    flow::cont(state + 1)
                } else {
                    flow::cont(state)
                }
            },
        )]
        struct TestVisitor;

        // The expressions are:
        // In the SELECT projection: a, b, 123, myfunc(b), b
        // In the WHERE clause: a, b, a > b, b, 100, b < 100, a > b AND b < 100
        // In the ORDER BY clause: a, b
        // Total: 14
        match ast.evaluate(&TestVisitor, 0) {
            Ok(count) => assert_eq!(14, count),
            err => panic!("{:#?}", err),
        };
    }

    #[test]
    fn useless_visits_are_avoided() {
        let dialect = dialect::GenericDialect {};
        let sql = "SELECT 1 as a WHERE a > 0";
        let ast = parser::Parser::parse_sql(&dialect, sql).unwrap();

        #[derive(Visitor)]
        #[visitor(
            state_ty = Vec<String>,
            enter = {
                // Types that should _not_ be visited because we know it'll be
                // None/empty with the `sql` expression below.
                if node.is::<Option<With>>() {
                    state.push("Option<With>".into());
                }
                if node.is::<Vec<TableWithJoins>>() {
                    state.push("Vec<TableWithJoins>".into());
                }

                // Types that _should_ be visited because we know they'll be present
                // after parsing the `sql` expression below.
                if node.is::<Option<Expr>>() {
                    state.push("Option<Expr>".into());
                }
                if node.is::<Vec<SelectItem>>() {
                    state.push("Vec<SelectItem>".into());
                }

                flow::cont(state)
            }
        )]
        struct TestVisitor;

        match ast.evaluate(&TestVisitor, Vec::<String>::new()) {
            Ok(mut state) => {
                state.sort();
                let mut expected_items = Vec::<String>::new();
                expected_items.push("Option<Expr>".to_string());
                expected_items.push("Vec<SelectItem>".to_string());
                assert_eq!(state, expected_items)
            }
            Err(err) => panic!("{:?}", err),
        };
    }

    #[test]
    fn source_node_reachable_fields_are_visited_first() {
        #[derive(Debug, Visitor)]
        #[visitor(
            state_ty = Vec<&'static str>,
            enter = {
                state.push(std::any::type_name::<N>());
                flow::cont(state)
            }
        )]
        struct TestVisitor;

        let dialect = dialect::GenericDialect {};

        let sql = "SELECT a, b, 123, myfunc(b) \
                   FROM table_1 \
                   WHERE a > b AND b < 100 \
                   ORDER BY a DESC, b";

        let ast = parser::Parser::parse_sql(&dialect, sql).unwrap();

        match ast.evaluate(&TestVisitor, Default::default()) {
            Ok(state) => assert_eq!(
                state[0..9],
                [
                    "alloc::vec::Vec<sqlparser::ast::Statement>",
                    "sqlparser::ast::Statement",
                    "sqlparser::ast::query::Query",
                    "sqlparser::ast::query::SetExpr",
                    "sqlparser::ast::query::Select",
                    "alloc::vec::Vec<sqlparser::ast::query::TableWithJoins>",
                    "sqlparser::ast::query::TableWithJoins",
                    "sqlparser::ast::query::TableFactor",
                    "sqlparser::ast::ObjectName"
                ]
            ),
            Err(err) => panic!("Err: {:?}", err),
        };
    }

    #[test]
    fn visitor_composition() {
        #[derive(Default, Debug)]
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

        #[derive(Default, Visitor)]
        #[visitor(
            state_ty = MyState,
            exit = { flow::cont(state.update_balanced()) }
        )]
        struct UpdateBalanced;

        #[derive(Default, Visitor)]
        #[visitor(
            state_ty = MyState,
            enter = { flow::cont(state.inc_enter_count()) },
            exit = { flow::cont(state.inc_exit_count()) },
        )]
        struct UpdateCounts;

        #[derive(Visitor)]
        #[visitor(
            state_ty = MyState,
            children = [
                UpdateBalanced,
                UpdateCounts
            ]
        )]
        struct BalanceCheck;

        let dialect = dialect::GenericDialect {};

        let sql = "SELECT a, b, 123, myfunc(b) \
                    FROM table_1 \
                    WHERE a > b AND b < 100 \
                    ORDER BY a DESC, b";

        let ast = parser::Parser::parse_sql(&dialect, sql).unwrap();

        match ast.evaluate(&BalanceCheck, MyState::default()) {
            Ok(state) => assert!(state.balanced),
            Err(err) => panic!("{:?}", err),
        };
    }
}
