//!
//! This crate implements an enhanced Visitor implementation suitable for
//! semantic analysis of SQL.
//!
//! The AST is provided by the `sqltk-parser` crate - which this crate re-exports.
//!
//! ## Key features
//!
//! 1. Full coverage of all AST node types from `sqltk-parser` (including all field types and container types (`Vec`, `Option` & `Box`)) and terminal nodes.
//!
//! 2. [`Transform`] trait methods do not receive a mutable node argument which means that non-mutable references AST nodes can be retained in your own data structures from previous analysis passes.
//!
//! ## Installation
//!
//! ```shell
//! $ cargo add sqltk
//! ```
//!
//! ## Example
//!
//! Count the number of Expr nodes in an AST.
//!
//! ```
//! use std::convert::Infallible;
//! use std::ops::ControlFlow;
//! use sqltk::{Visitor, Visitable, into_result, Break};
//! use sqltk_parser::{ast::Expr, dialect::GenericDialect, parser::Parser};
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
//! struct CountOfExprNodes{ count: usize };
//!
//! impl<'ast> Visitor<'ast> for CountOfExprNodes {
//!     type Error = Infallible;
//!
//!     fn enter<N: Visitable>(&mut self, node: &'ast N) -> ControlFlow<Break<Self::Error>> {
//!         if let Some(expr) = node.downcast_ref::<Expr>() {
//!            self.count += 1;
//!         }
//!         ControlFlow::Continue(())
//!     }
//! }
//!
//! // The expressions are:
//! //   In the SELECT projection: a, b, 123, myfunc(b), b
//! //   In the WHERE clause: a, b, a > b, b, 100, b < 100, a > b AND b < 100
//! //   In the ORDER BY clause: a, b
//! // Total: 14
//!
//! let mut visitor = CountOfExprNodes{ count: 0 };
//!
//! match into_result(ast.accept(&mut visitor)) {
//!     Ok(()) => assert_eq!(14, visitor.count),
//!     err => panic!("{:#?}", err)
//! };
//! ```

// This module re-exports sqltk-core and is where the
// crate-level tests live.
//
// No functionality should be created here (beyond simply re-exporting).

mod generated;
mod node_key;
mod transform;
mod transformable_impls;
mod visitable_impls;

// Re-export sqltk-parser under this lib as `sqltk::parser`, so we can avoid
// needing to expose the machinations of the versioning of our
// forked-but-with-upstream-updates sqlparser.
pub mod parser {
    pub use sqltk_parser::*;
}

pub use node_key::*;
use sqltk_parser::ast::{Expr, ObjectName, Statement};
pub use transform::*;

use core::fmt::Debug;
use std::{any::Any, ops::ControlFlow};

/// Trait for types that can visit any `sqltk-parser` AST node.
///
/// The reason for this is to support implementations that can store a reference to an AST node: `&'ast N` and also
/// support recursive AST traversal. With a `&mut self` the former can be supported but the latter cannot because the
/// a mutable borrow would be required to have the lifetime of `'ast` which prevents recursion.
///
/// ## `'ast`
///
/// The lifetime of the abstract syntax tree.
///
/// # The `N: Visitable` bound
///
/// This is required so that implementations can cast `N` to the AST node types that they are interested in.
#[allow(unused_variables)]
pub trait Visitor<'ast>
where
    Self: Sized,
{
    type Error;

    /// Invoked when `node` is entered and before children of `node` are visited (parent nodes are entered before child
    /// nodes).
    fn enter<N: Visitable>(&mut self, node: &'ast N) -> ControlFlow<Break<Self::Error>> {
        ControlFlow::Continue(())
    }

    /// Invoked when `node` is exited and after children of `node` have been visited (child nodes are exited before
    /// parent nodes).
    fn exit<N: Visitable>(&mut self, node: &'ast N) -> ControlFlow<Break<Self::Error>> {
        ControlFlow::Continue(())
    }
}

/// Trait for types that can be traversed by a [`Visitor`].
///
/// All required implementations of this trait (every `sqltk-parser` AST node type) are provided.
pub trait Visitable
where
    Self: 'static + Sized + AsNodeKey,
{
    /// Accepts a borrowed [`Visitor`] and traverses the AST starting at `self` invoking [`Visitor::enter`] and
    /// [`Visitor::exit`] as nodes are entered and exiting respectively.
    fn accept<'ast, V: Visitor<'ast>>(&'ast self, visitor: &mut V) -> ControlFlow<Break<V::Error>>;

    /// Tries to downcast `self` as `&Target`.
    fn downcast_ref<Target: Visitable>(&self) -> Option<&Target> {
        (self as &dyn Any).downcast_ref::<Target>()
    }

    /// Tries to downcast `self` as `&mut Target`.
    fn downcast_mut<Target: Visitable>(&mut self) -> Option<&mut Target> {
        (self as &mut dyn Any).downcast_mut::<Target>()
    }

    /// Returns `true` if the inner type is the same as `Target`.
    fn is<Target: Visitable>(&self) -> bool {
        (self as &dyn Any).is::<Target>()
    }
}

/// Type used to signal abnormal control flow from a [`Visitor`] during AST traversal.
#[derive(Debug)]
pub enum Break<E> {
    /// Do not visit child nodes of the current node and resume traversal from the next sibling of the current node.
    /// This is useful to save CPU cycles when an exhaustive traversal of an AST is not required.
    ///
    /// This variant has no effect when returned from [`Visitor::exit`]; instead it is treated the same as `Continue`.
    SkipChildren,

    /// Ends traversal entirely but completes successfully. Useful to force an early return when an exhaustive traversal
    /// of the AST is unnecessary.  An example use-case would be implementing a search over the AST that returns as soon
    /// as the target has been found.
    Finished,

    /// An error occurred. Traversal will be aborted.
    Err(E),
}

impl<E1> Break<E1> {
    pub fn convert<E2>(self) -> Break<E2>
    where
        E2: From<E1>,
    {
        match self {
            Break::SkipChildren => Break::SkipChildren,
            Break::Finished => Break::Finished,
            Break::Err(e1) => Break::Err(E2::from(e1)),
        }
    }
}

/// Converts a `Result<(), E>` into a `ControlFlow<Break<E>, ()>`.
pub fn into_control_flow<T, E>(result: Result<T, E>) -> ControlFlow<Break<E>> {
    match result {
        Ok(_) => ControlFlow::Continue(()),
        Err(err) => ControlFlow::Break(Break::Err(err)),
    }
}

/// Converts a `ControlFlow<Break<E1>, ()>` into a `ControlFlow<Break<E2>, ()>`.
pub fn convert_control_flow<E1, E2>(flow: ControlFlow<Break<E1>>) -> ControlFlow<Break<E2>>
where
    E2: From<E1>,
{
    match flow {
        ControlFlow::Break(Break::Err(e1)) => ControlFlow::Break(Break::Err(E2::from(e1))),
        ControlFlow::Break(Break::SkipChildren) => ControlFlow::Break(Break::SkipChildren),
        ControlFlow::Break(Break::Finished) => ControlFlow::Break(Break::Finished),
        ControlFlow::Continue(()) => ControlFlow::Continue(()),
    }
}

/// Converts a `ControlFlow<Break<E>, ()>` into a `Result<(), E>`.
pub fn into_result<E>(flow: ControlFlow<Break<E>>) -> Result<(), E> {
    match flow {
        ControlFlow::Continue(()) => Ok(()),
        ControlFlow::Break(Break::SkipChildren) => Ok(()),
        ControlFlow::Break(Break::Finished) => Ok(()),
        ControlFlow::Break(Break::Err(err)) => Err(err),
    }
}

impl<E> PartialEq for Break<E> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::SkipChildren, Self::SkipChildren) => true,
            (Self::Finished, Self::Finished) => true,
            (Self::Err(_), Self::Err(_)) => false,
            _ => false,
        }
    }
}

pub fn visit_statements<N, E, F>(node: &N, f: F) -> ControlFlow<Break<E>>
where
    N: Visitable,
    F: Fn(&Statement) -> ControlFlow<Break<E>>,
{
    let mut visitor = StatementVisitor(f);
    node.accept(&mut visitor)?;
    ControlFlow::Continue(())
}

struct StatementVisitor<F>(F);

impl<'ast, F, E> Visitor<'ast> for StatementVisitor<F>
where
    F: Fn(&Statement) -> ControlFlow<Break<E>>,
{
    type Error = E;

    fn enter<N: Visitable>(&mut self, node: &'ast N) -> ControlFlow<Break<Self::Error>> {
        if let Some(statement) = node.downcast_ref::<Statement>() {
            return self.0(statement);
        }
        ControlFlow::Continue(())
    }

    fn exit<N: Visitable>(&mut self, node: &'ast N) -> ControlFlow<Break<Self::Error>> {
        if let Some(statement) = node.downcast_ref::<Statement>() {
            return self.0(statement);
        }
        ControlFlow::Continue(())
    }
}

pub fn visit_expressions<N, E, F>(node: &N, f: F) -> ControlFlow<Break<E>>
where
    N: Visitable,
    F: Fn(&Expr) -> ControlFlow<Break<E>>,
{
    let mut visitor = ExprVisitor(f);
    node.accept(&mut visitor)?;
    ControlFlow::Continue(())
}

struct ExprVisitor<F>(F);

impl<'ast, F, E> Visitor<'ast> for ExprVisitor<F>
where
    F: Fn(&Expr) -> ControlFlow<Break<E>>,
{
    type Error = E;

    fn enter<N: Visitable>(&mut self, node: &'ast N) -> ControlFlow<Break<Self::Error>> {
        if let Some(expr) = node.downcast_ref::<Expr>() {
            return self.0(expr);
        }
        ControlFlow::Continue(())
    }

    fn exit<N: Visitable>(&mut self, node: &'ast N) -> ControlFlow<Break<Self::Error>> {
        if let Some(expr) = node.downcast_ref::<Expr>() {
            return self.0(expr);
        }
        ControlFlow::Continue(())
    }
}

pub fn visit_relations<N, E, F>(node: &N, f: F) -> ControlFlow<Break<E>>
where
    N: Visitable,
    F: Fn(&ObjectName) -> ControlFlow<Break<E>>,
{
    let mut visitor = ObjectNameVisitor(f);
    node.accept(&mut visitor)?;
    ControlFlow::Continue(())
}

struct ObjectNameVisitor<F>(F);

impl<'ast, F, E> Visitor<'ast> for ObjectNameVisitor<F>
where
    F: Fn(&ObjectName) -> ControlFlow<Break<E>>,
{
    type Error = E;

    fn enter<N: Visitable>(&mut self, node: &'ast N) -> ControlFlow<Break<Self::Error>> {
        if let Some(object_name) = node.downcast_ref::<ObjectName>() {
            return self.0(object_name);
        }
        ControlFlow::Continue(())
    }

    fn exit<N: Visitable>(&mut self, node: &'ast N) -> ControlFlow<Break<Self::Error>> {
        if let Some(object_name) = node.downcast_ref::<ObjectName>() {
            return self.0(object_name);
        }
        ControlFlow::Continue(())
    }
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, convert::Infallible, fmt::Debug, ops::ControlFlow, rc::Rc};

    use super::{into_result, Break, Visitable, Visitor};
    use sqltk_parser::{
        ast::{Expr, SelectItem, TableWithJoins, With},
        dialect, parser,
    };

    #[test]
    fn basic() {
        let dialect = dialect::GenericDialect {};

        let sql = "SELECT a, b, 123, myfunc(b) \
                   FROM table_1 \
                   WHERE a > b AND b < 100 \
                   ORDER BY a DESC, b";

        let ast = parser::Parser::parse_sql(&dialect, sql).unwrap();

        #[derive(Debug)]
        struct TestVisitor(usize);

        impl<'ast> Visitor<'ast> for TestVisitor {
            type Error = Infallible;

            fn enter<N: Visitable>(&mut self, node: &'ast N) -> ControlFlow<Break<Self::Error>> {
                if node.is::<Expr>() {
                    self.0 += 1;
                }
                ControlFlow::Continue(())
            }
        }

        let mut visitor = TestVisitor(0);

        // The expressions are:
        // In the SELECT projection: a, b, 123, myfunc(b), b
        // In the WHERE clause: a, b, a > b, b, 100, b < 100, a > b AND b < 100
        // In the ORDER BY clause: a, b
        // Total: 14
        match into_result(ast.accept(&mut visitor)) {
            Ok(()) => assert_eq!(visitor.0, 14),
            _ => panic!(),
        };
    }

    #[test]
    fn useless_visits_are_avoided() {
        let dialect = dialect::GenericDialect {};
        let sql = "SELECT 1 as a WHERE a > 0";
        let ast = parser::Parser::parse_sql(&dialect, sql).unwrap();

        struct TestVisitor(Vec<String>);

        impl<'ast> Visitor<'ast> for TestVisitor {
            type Error = Infallible;

            fn enter<N: Visitable>(&mut self, node: &'ast N) -> ControlFlow<Break<Self::Error>> {
                // Types that should _not_ be visited because we know it'll be
                // None/empty with the `sql` expression below.
                if node.is::<Option<With>>() {
                    self.0.push("Option<With>".into());
                }
                if node.is::<Vec<TableWithJoins>>() {
                    self.0.push("Vec<TableWithJoins>".into());
                }

                // Types that _should_ be visited because we know they'll be present
                // after parsing the `sql` expression below.
                if node.is::<Option<Expr>>() {
                    self.0.push("Option<Expr>".into());
                }
                if node.is::<Vec<SelectItem>>() {
                    self.0.push("Vec<SelectItem>".into());
                }

                ControlFlow::Continue(())
            }
        }

        let mut visitor = TestVisitor(Vec::new());

        match into_result(ast.accept(&mut visitor)) {
            Ok(()) => {
                visitor.0.sort();
                assert_eq!(
                    &visitor.0,
                    &["Option<Expr>".to_string(), "Vec<SelectItem>".to_string()]
                )
            }
            Err(err) => panic!("{:?}", err),
        };
    }

    #[test]
    fn source_node_reachable_fields_are_visited_first() {
        #[derive(Debug)]
        struct TestVisitor(Vec<&'static str>);

        impl<'ast> Visitor<'ast> for TestVisitor {
            type Error = Infallible;

            fn enter<N: 'static>(&mut self, _node: &'ast N) -> ControlFlow<Break<Self::Error>> {
                self.0.push(std::any::type_name::<N>());
                ControlFlow::Continue(())
            }
        }

        let dialect = dialect::GenericDialect {};

        let sql = "SELECT a, b, 123, myfunc(b) \
                   FROM table_1 \
                   WHERE a > b AND b < 100 \
                   ORDER BY a DESC, b";

        let ast = parser::Parser::parse_sql(&dialect, sql).unwrap();

        let mut visitor = TestVisitor(Vec::new());

        match into_result(ast.accept(&mut visitor)) {
            Ok(()) => assert_eq!(
                visitor.0[0..9],
                [
                    "alloc::vec::Vec<sqltk_parser::ast::Statement>",
                    "sqltk_parser::ast::Statement",
                    "sqltk_parser::ast::query::Query",
                    "sqltk_parser::ast::query::SetExpr",
                    "sqltk_parser::ast::query::Select",
                    "alloc::vec::Vec<sqltk_parser::ast::query::TableWithJoins>",
                    "sqltk_parser::ast::query::TableWithJoins",
                    "sqltk_parser::ast::query::TableFactor",
                    "sqltk_parser::ast::ObjectName"
                ]
            ),
            Err(err) => panic!("Err: {:?}", err),
        };
    }

    #[test]
    fn visitor_composition() {
        struct CountEnterAndExit {
            enter_count: usize,
            exit_count: usize,
        }

        impl<'ast> Visitor<'ast> for CountEnterAndExit {
            type Error = Infallible;

            fn enter<N: Visitable>(&mut self, _node: &'ast N) -> ControlFlow<Break<Self::Error>> {
                self.enter_count += 1;
                ControlFlow::Continue(())
            }

            fn exit<N: Visitable>(&mut self, _node: &'ast N) -> ControlFlow<Break<Self::Error>> {
                self.exit_count += 1;
                ControlFlow::Continue(())
            }
        }

        struct CheckBalanced {
            counter: Rc<RefCell<CountEnterAndExit>>,
            pub is_balanced: bool,
        }

        impl<'ast> Visitor<'ast> for CheckBalanced {
            type Error = Infallible;

            fn exit<N: Visitable>(&mut self, _node: &'ast N) -> ControlFlow<Break<Self::Error>> {
                self.is_balanced =
                    self.counter.borrow().enter_count == self.counter.borrow().exit_count;
                ControlFlow::Continue(())
            }
        }

        struct Main {
            count_enter_and_exit: Rc<RefCell<CountEnterAndExit>>,
            check_balanced: Rc<RefCell<CheckBalanced>>,
        }

        impl<'ast> Visitor<'ast> for Main {
            type Error = Infallible;

            fn enter<N: Visitable>(&mut self, node: &'ast N) -> ControlFlow<Break<Self::Error>> {
                self.check_balanced.borrow_mut().enter(node)?;
                self.count_enter_and_exit.borrow_mut().enter(node)
            }

            fn exit<N: Visitable>(&mut self, node: &'ast N) -> ControlFlow<Break<Self::Error>> {
                self.count_enter_and_exit.borrow_mut().exit(node)?;
                self.check_balanced.borrow_mut().exit(node)
            }
        }

        let counter = Rc::new(RefCell::new(CountEnterAndExit {
            enter_count: 0,
            exit_count: 0,
        }));

        let mut visitor = Main {
            count_enter_and_exit: counter.clone(),
            check_balanced: Rc::new(RefCell::new(CheckBalanced {
                counter: counter.clone(),
                is_balanced: false,
            })),
        };

        let dialect = dialect::GenericDialect {};

        let sql = "SELECT a, b, 123, myfunc(b) \
                    FROM table_1 \
                    WHERE a > b AND b < 100 \
                    ORDER BY a DESC, b";

        let ast = parser::Parser::parse_sql(&dialect, sql).unwrap();

        match into_result(ast.accept(&mut visitor)) {
            Ok(()) => assert!(visitor.check_balanced.borrow().is_balanced),
            Err(err) => panic!("{:?}", err),
        };
    }
}
