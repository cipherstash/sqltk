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
//! let visitor = Expr::on_enter(|_, count| flow::infallible::cont(count + 1));
//!
//! // The expressions are:
//! //   In the SELECT projection: a, b, 123, myfunc(b), b
//! //   In the WHERE clause: a, b, a > b, b, 100, b < 100, a > b AND b < 100
//! //   In the ORDER BY clause: a, b
//! // Total: 14
//!
//! match ast.evaluate(&visitor, 0) {
//!     Ok(count) => assert_eq!(14, count),
//!     err => panic!("{:#?}", err)
//! };
//! ```

mod dispatch;
mod node;
mod visitable_impls;
mod visitor_stack;

pub mod flow;
pub mod prelude;

pub use dispatch::*;
pub use node::*;
pub use visitor_stack::*;

// Re-export sqlparser
pub use sqlparser;
// Re-export bigdecimal
pub use bigdecimal;

use core::{convert::Infallible, fmt::Debug};
use std::{error::Error, ops::ControlFlow};

/// Trait for types that can visit any `sqlparser` AST node.
///
/// A "visit" of a single AST node is a call to `enter` followed some time later
/// with a call to `exit` if no error was returned by `enter`.
///
/// Visitors are parameterised by a `State` and an error `E`. If unspecified,
/// `E` is [`Infallible`] which means an error cannot be returned by `enter` or
/// `exit`.
pub trait Visitor<'ast, State, E = Infallible>
where
    E: Error + Debug,
{
    /// Called when node is entered.
    ///
    /// `node` is a reference to any `sqlparser` AST node type.
    ///
    /// Ownership of `state` is passed to `enter`. Typically `state` will be
    /// mutated and returned to the caller in the [`VisitorControlFlow`] value,
    /// but the implementation is free to drop the original state and return a
    /// new value should it wish to do so.
    fn enter<N>(&self, node: &'ast N, state: State) -> VisitorControlFlow<'ast, State, E>
    where
        N: 'static + Visitable<'ast>,
        &'ast N: Into<Node<'ast>>;

    /// Called when node is exited.
    ///
    /// `node` is a reference to any `sqlparser` AST node type.
    ///
    /// Ownership of `state` is passed to `exit`. Typically `state` will be
    /// mutated and returned to the caller in the [`VisitorControlFlow`] value,
    /// but the implementation is free to drop the original state and return a
    /// new value should it wish to do so.
    fn exit<N>(&self, node: &'ast N, state: State) -> VisitorControlFlow<'ast, State, E>
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
    fn accept<State, E, V>(
        &'ast self,
        visitor: &V,
        state: State,
    ) -> VisitorControlFlow<'ast, State, E>
    where
        E: Error + Debug,
        V: Visitor<'ast, State, E>;

    /// Traverses the AST invoking the `enter` & `exit` methods on `visitor`
    /// in order to compute a new state.
    ///
    /// Returns `Ok(state)` on success, or `Err((err, state))` if an error
    /// occurs.
    fn evaluate<State, V, E>(&'ast self, visitor: &V, state: State) -> Result<State, (E, State)>
    where
        E: Error + Debug,
        V: Visitor<'ast, State, E>,
    {
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
    fn on_enter<State, E, FnEnter>(enter_fn: FnEnter) -> impl Visitor<'ast, State, E>
    where
        State: 'ast,
        E: Error + Debug,
        // FnEnter: 'ast + Fn(&'ast Self, State) -> VisitorControlFlow<'ast, State, E>,
        FnEnter: Fn(&'ast Self, State) -> VisitorControlFlow<'ast, State, E>,
    {
        Self::on_enter_exit(enter_fn, ignore())
    }

    /// Returns a [`Visitor`] impl whose `exit` method will invoke
    /// `exit_fn` for nodes of type `Self`.
    fn on_exit<State, E, FnExit>(exit_fn: FnExit) -> impl Visitor<'ast, State, E>
    where
        State: 'ast,
        E: Error + Debug,
        FnExit: Fn(&'ast Self, State) -> VisitorControlFlow<'ast, State, E>,
    {
        Self::on_enter_exit(ignore(), exit_fn)
    }

    /// Returns a [`Visitor`] impl whose `enter` & `exit` methods will
    /// invoke `enter_fn` & `exit_fn` for nodes of type `Self`.
    fn on_enter_exit<FnEnter, FnExit, State, E>(
        enter_fn: FnEnter,
        exit_fn: FnExit,
    ) -> impl Visitor<'ast, State, E>
    where
        State: 'ast,
        E: Error + Debug,
        FnEnter: Fn(&'ast Self, State) -> VisitorControlFlow<'ast, State, E>,
        FnExit: Fn(&'ast Self, State) -> VisitorControlFlow<'ast, State, E>,
    {
        DowncastDispatcher::<'ast, Self, _, _, _, _>::new(enter_fn, exit_fn)
    }
}

/// Type used to signal abnormal control flow from a [`Visitor`] during
/// AST traversal.
///
/// All variants propagate the `State` back up the call stack.
#[derive(Debug)]
pub enum Break<State, E>
where
    E: Error + Debug,
{
    /// Do not visit child nodes of the current node and resume traversal
    /// from the next sibling of the current node. This is useful to save CPU
    /// cycles when an exhaustive traversal of an AST is not required.
    SkipChildren(State),

    /// Aborts traversal entirely - but completes traversal successfully. Useful
    /// to force an early return when an exhaustive traversal of the AST is
    /// unnecessary.
    Finished(State),

    /// An error occurred.
    Err(E, State),
}

impl<'ast, State, E> PartialEq for Break<State, E>
where
    E: Error + Debug,
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

impl<State, E> Eq for Break<State, E>
where
    E: Error + Debug,
    State: Eq,
{
}

/// [`ControlFlow`] type alias for values returned by [`Visitor::enter`] &
/// [`Visitor::exit`].
pub type VisitorControlFlow<'ast, State, E> = ControlFlow<Break<State, E>, State>;

#[cfg(test)]
pub mod test {
    use core::convert::Infallible;
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

        let visitor = Expr::on_enter::<_, Infallible, _>(|_, count| flow::cont(count + 1));

        // The expressions are:
        // In the SELECT projection: a, b, 123, myfunc(b), b
        // In the WHERE clause: a, b, a > b, b, 100, b < 100, a > b AND b < 100
        // In the ORDER BY clause: a, b
        // Total: 14
        match ast.evaluate(&visitor, 0) {
            Ok(count) => assert_eq!(14, count),
            err => panic!("{:#?}", err),
        };
    }

    #[test]
    fn fallback() {
        let ast = String::from("OH HAI!");

        assert_eq!(
            VisitorControlFlow::Continue(0),
            ast.accept(
                &Expr::on_enter::<_, Infallible, _>(|_, count| flow::cont(count + 1)),
                0
            )
        );
    }

    #[test]
    fn useless_visits_are_avoided() {
        let dialect = dialect::GenericDialect {};
        let sql = "SELECT 1 as a WHERE a > 0";
        let ast = parser::Parser::parse_sql(&dialect, sql).unwrap();

        let mut stack = VisitorStack::<'_, Vec<String>, Infallible>::new();
        // Types that should _not_ be visited because we know it'll be
        // None/empty with the `sql` expression below.
        stack.push(Option::<With>::on_enter(flow::infallible::modify_state(
            |state: &mut Vec<String>| state.push("Option<With>".into()),
        )));
        stack.push(Vec::<TableWithJoins>::on_enter(
            flow::infallible::modify_state(|state: &mut Vec<String>| {
                state.push("Vec<TableWithJoins>".into())
            }),
        ));
        // Types that _should_ be visited because we know they'll be present
        // after parsing the `sql` expression below.
        stack.push(Option::<Expr>::on_enter(flow::infallible::modify_state(
            |state: &mut Vec<String>| state.push("Option<Expr>".into()),
        )));
        stack.push(Vec::<SelectItem>::on_enter(flow::infallible::modify_state(
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
            Err((err, _)) => panic!("{:?}", err),
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

        let mut stack = VisitorStack::<_, Infallible>::new();
        stack.push(AnyNode::on_enter::<_, Infallible, _>(
            |any_node, mut state: Vec<String>| {
                state.push(any_node.to_string());
                flow::cont(state)
            },
        ));

        match ast.evaluate(&stack, state) {
            Ok(state) => assert_eq!(
                state[0..9],
                [
                    "Vec<Statement>",
                    "Statement",
                    "Query",
                    "SetExpr",
                    "Select",
                    "Vec<TableWithJoins>",
                    "TableWithJoins",
                    "TableFactor",
                    "ObjectName",
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

        let mut stack = VisitorStack::<_, Infallible>::new();
        stack.push(Expr::on_exit(|_, state: MyState| {
            flow::infallible::cont(state.update_balanced())
        }));
        stack.push(Expr::on_enter_exit(
            |_, state: MyState| flow::infallible::cont(state.inc_enter_count()),
            |_, state: MyState| flow::infallible::cont(state.inc_exit_count()),
        ));

        match ast.evaluate(&stack, MyState::default()) {
            Ok(state) => assert!(state.balanced),
            Err((err, _)) => panic!("{:?}", err),
        };
    }

    #[test]
    fn on_enter_helper() {
        let ast = String::from("0123456789");

        let state = 0usize;

        let inc_counter = |_, state| flow::cont(state + 1);

        let result = ast.accept(&String::on_enter::<_, Infallible, _>(inc_counter), state);

        assert!(matches!(result, ControlFlow::Continue(1)));
    }
}
