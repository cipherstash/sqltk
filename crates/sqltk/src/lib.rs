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
//!    traversal (see: [`sqltk_analysis`])
//!
//! 4. An API for composing [`Visitor`] implementations to facilitate complex
//!    SQL analysis scenarios made from small, unit testable pieces
//!    (see: [`sqltk_analysis`]).
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
//! let visitor = Expr::on_enter(|_, count| flow::cont(count + 1));
//!
//! // The expressions are:
//! //   In the SELECT projection: a, b, 123, myfunc(b), b
//! //   In the WHERE clause: a, b, a > b, b, 100, b < 100, a > b AND b < 100
//! //   In the ORDER BY clause: a, b
//! // Total: 14
//!
//! match ast.evaluate(&visitor, 0) {
//!     Ok(count) => assert_eq!(14, count),
//!     _ => assert!(false)
//! };
//! ```

mod dispatch;
mod node;
mod visitable_impls;
mod visitor_stack;

pub mod prelude;
pub mod flow;

pub use dispatch::*;
pub use node::*;
pub use visitor_stack::*;

// Re-export sqlparser
pub use sqlparser;
// Re-export bigdecimal
pub use bigdecimal;

use std::{error::Error, ops::ControlFlow};

/// Trait for types that can visit any `sqlparser` AST node.
///
/// A "visit" of a single AST node is a call to `enter` paired with a call to `exit`.
pub trait Visitor<'ast, State> {
    /// Called when node is entered.
    fn enter<N>(&self, node: &'ast N, state: State) -> VisitorControlFlow<'ast, State>
    where
        N: 'static + Visitable<'ast>,
        &'ast N: Into<Node<'ast>>;

    /// Called when a node is exited.
    fn exit<N>(&self, node: &'ast N, state: State) -> VisitorControlFlow<'ast, State>
    where
        N: 'static + Visitable<'ast>,
        &'ast N: Into<Node<'ast>>;
}

/// Trait for types that can be visited by a [`Visitor`].
/// This trait is implemented for all AST nodes defined by `sqlparser`.
pub trait Visitable<'ast>
where
    Self: 'static + Sized,
{
    /// Entry point to begin AST traversal with a [`Visitor`].
    ///
    /// Invokes [`Visitor::enter`] and [`Visitor::exit`] for
    /// every AST node encountered during traversal with the exception that if
    /// the `Visitor` returns
    /// [`VisitorControlFlow::Continue(Nav::Skip)`], remaining children of the
    /// current node will be skipped.
    fn accept<State, VD: Visitor<'ast, State>>(
        &'ast self,
        visitor: &VD,
        state: State,
    ) -> VisitorControlFlow<'ast, State>;

    /// Traverse the AST invoking the `enter` & `exit` methods on `dispatcher`
    /// in order to compute a new state.
    ///
    /// Returns `Ok(state)` on success, or `Err((err, state))` if an error
    /// occurs.
    fn evaluate<State, VD: Visitor<'ast, State>>(
        &'ast self,
        visitor: &VD,
        state: State,
    ) -> Result<State, (Box<(dyn 'ast + Error)>, State)> {
        match self.accept(visitor, state) {
            VisitorControlFlow::Continue(state) => Ok(state),
            VisitorControlFlow::Break(brk) => match brk {
                Break::Finished(state) => Ok(state),
                Break::SkipChildren(state) => Ok(state),
                Break::Err(err, state) => Err((err, state)),
            },
        }
    }

    /// Returns a [`Visitor`] impl whose `enter` method will invoke
    /// `enter_fn` for nodes of type `Self`.
    fn on_enter<FnEnter, State>(enter_fn: FnEnter) -> impl Visitor<'ast, State>
    where
        State: 'ast,
        FnEnter: 'ast + Fn(&'ast Self, State) -> VisitorControlFlow<'ast, State>,
    {
        Self::on_enter_exit(enter_fn, ignore())
    }

    /// Returns a [`Visitor`] impl whose `exit` method will invoke
    /// `exit_fn` for nodes of type `Self`.
    fn on_exit<FnExit, State>(exit_fn: FnExit) -> impl Visitor<'ast, State>
    where
        State: 'ast,
        FnExit: 'ast + Fn(&'ast Self, State) -> VisitorControlFlow<'ast, State>,
    {
        Self::on_enter_exit(ignore(), exit_fn)
    }

    /// Returns a [`Visitor`] impl whose `enter` & `exit` methods will
    /// invoke `enter_fn` & `exit_fn` for nodes of type `Self`.
    fn on_enter_exit<FnEnter, FnExit, State>(
        enter_fn: FnEnter,
        exit_fn: FnExit,
    ) -> impl Visitor<'ast, State>
    where
        State: 'ast,
        FnEnter: Fn(&'ast Self, State) -> VisitorControlFlow<'ast, State>,
        FnExit: Fn(&'ast Self, State) -> VisitorControlFlow<'ast, State>,
    {
        DowncastDispatcher::<'ast, Self, _, _, _>::new(enter_fn, exit_fn)
    }
}

/// Type used to signal abnormal control flow from a [`Visitor`] during
/// AST traversal.
///
/// All variants propagate the `State` back up the call stack.
#[derive(Debug)]
pub enum Break<'ast, State> {
    /// Do not visit child nodes of the current node and resume traversal
    /// from the next sibling of the current node. This is useful to save CPU
    /// cycles when an exhaustive traversal of an AST is not required.
    SkipChildren(State),

    /// Aborts traversal entirely - but completes traversal successfully. Useful
    /// to force an early return when an exhaustive traversal of the AST is
    /// unnecessary.
    Finished(State),

    /// An error occurred.
    /// TODO: consider being generic over the error type.
    Err(Box<dyn Error + 'ast>, State),
}

impl<'ast, State> PartialEq for Break<'ast, State>
where
    State: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::SkipChildren(l0), Self::SkipChildren(r0)) => l0 == r0,
            (Self::Finished(l0), Self::Finished(r0)) => l0 == r0,
            (Self::Err(_, _), Self::Err(_, _)) => false,
            _ => false,
        }
    }
}

impl<'ast, State> Eq for Break<'ast, State> where State: Eq {}

/// [`ControlFlow`] type alias for values returned by [`Visitor::enter`] &
/// [`Visitor::exit`].
pub type VisitorControlFlow<'ast, State> = ControlFlow<Break<'ast, State>, State>;


#[cfg(test)]
pub mod test {
    use std::ops::ControlFlow;

    use crate::{
        flow,
        sqlparser::ast::{Expr, SelectItem, TableWithJoins, With},
        sqlparser::{dialect, parser},
        AnyNode, Visitable, VisitorControlFlow, VisitorStack,
    };

    #[test]
    fn basic() {
        let dialect = dialect::GenericDialect {};

        let sql = "SELECT a, b, 123, myfunc(b) \
                   FROM table_1 \
                   WHERE a > b AND b < 100 \
                   ORDER BY a DESC, b";

        let ast = parser::Parser::parse_sql(&dialect, sql).unwrap();

        let visitor = Expr::on_enter(|_, count| flow::cont(count + 1));

        // The expressions are:
        // In the SELECT projection: a, b, 123, myfunc(b), b
        // In the WHERE clause: a, b, a > b, b, 100, b < 100, a > b AND b < 100
        // In the ORDER BY clause: a, b
        // Total: 14
        match ast.evaluate(&visitor, 0) {
            Ok(count) => assert_eq!(14, count),
            _ => assert!(false)
        };
    }

    #[test]
    fn fallback() {
        let ast = String::from("OH HAI!");

        assert_eq!(
            VisitorControlFlow::Continue(0),
            ast.accept(&Expr::on_enter(|_, count| flow::cont(count + 1)), 0)
        );
    }

    #[test]
    fn useless_visits_are_avoided() {
        let dialect = dialect::GenericDialect {};
        let sql = "SELECT 1 as a WHERE a > 0";
        let ast = parser::Parser::parse_sql(&dialect, sql).unwrap();

        let stack = VisitorStack::<'_, Vec<String>>::new()
            // Types that should _not_ be visited because we know it'll be
            // None/empty with the `sql` expression below.
            .push(Option::<With>::on_enter(flow::modify_state(
                |state: &mut Vec<String>| state.push("Option<With>".into()),
            )))
            .push(Vec::<TableWithJoins>::on_enter(flow::modify_state(
                |state: &mut Vec<String>| state.push("Vec<TableWithJoins>".into()),
            )))
            // Types that _should_ be visited because we know they'll be present
            // after parsing the `sql` expression below.
            .push(Option::<Expr>::on_enter(flow::modify_state(
                |state: &mut Vec<String>| state.push("Option<Expr>".into()),
            )))
            .push(Vec::<SelectItem>::on_enter(flow::modify_state(
                |state: &mut Vec<String>| state.push("Vec<SelectItem>".into()),
            )));

        match ast.evaluate(&stack, Vec::<String>::new()) {
            Ok(mut state) => {
                state.sort();
                let mut expected_items = Vec::<String>::new();
                expected_items.push("Option<Expr>".to_string());
                expected_items.push("Vec<SelectItem>".to_string());
                assert_eq!(state, expected_items)
            }
            Err((err, _)) => assert!(false, "{:?}", err),
        };
    }

    #[test]
    fn source_node_reachable_fields_are_visited_first() {
        let dialect = dialect::GenericDialect {};

        let sql = "SELECT a, b, 123, myfunc(b) \
                   FROM table_1 \
                   WHERE a > b AND b < 100 \
                   ORDER BY a DESC, b";

        let ast = parser::Parser::parse_sql(&dialect, sql).unwrap();

        let state = Vec::<String>::new();

        let stack =
            VisitorStack::new().push(AnyNode::on_enter(|any_node, mut state: Vec<String>| {
                state.push(any_node.to_string());
                flow::cont(state)
            }));

        match ast.evaluate(&stack, state) {
            Ok(state) => assert_eq!(
                state[0..9],
                [
                    "Node::Vec(VecOf::Statement(Statement))",
                    "Node::Statement(Statement)",
                    "Node::Query(Query)",
                    "Node::SetExpr(SetExpr)",
                    "Node::Select(Select)",
                    "Node::Vec(VecOf::TableWithJoins(TableWithJoins))",
                    "Node::TableWithJoins(TableWithJoins)",
                    "Node::TableFactor(TableFactor)",
                    "Node::ObjectName(ObjectName)",
                ]
            ),
            Err((err, _)) => panic!("Err: {:?}", err),
        };
    }

    #[test]
    fn visitor_composition_with_visitor_stack() {
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

        let dialect = dialect::GenericDialect {};

        let sql = "SELECT a, b, 123, myfunc(b) \
                    FROM table_1 \
                    WHERE a > b AND b < 100 \
                    ORDER BY a DESC, b";

        let ast = parser::Parser::parse_sql(&dialect, sql).unwrap();

        let stack = VisitorStack::new()
            .push(Expr::on_enter_exit(
                |_, state: MyState| flow::cont(state.inc_enter_count()),
                |_, state: MyState| flow::cont(state.inc_exit_count()),
            ))
            .push(Expr::on_exit(|_, state: MyState| {
                flow::cont(state.update_balanced())
            }));

        match ast.evaluate(&stack, MyState::default()) {
            Ok(state) => assert!(state.balanced),
            Err((err, _)) => assert!(false, "{:?}", err),
        };
    }

    #[test]
    fn on_enter_helper() {
        let ast = String::from("0123456789");

        let state = 0usize;

        let inc_counter = |_, state| flow::cont(state + 1);

        let result = ast.accept(&String::on_enter(inc_counter), state);

        assert!(matches!(result, ControlFlow::Continue(1)));
    }
}
