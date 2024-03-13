//! When implementing a [`Visitor`] for a user-defined type it is not necessary
//! to manually implement `Visitor` for _all_ `sqlparser` AST node types.
//!
//! Rust makes this difficult to achieve because there is no language capability
//! in stable Rust that allows one to define a default-yet-overidable blanket
//! implementation of a trait. Such a feature is eventually coming, but the
//! current implementation in Rust nightly (min_specialization) has soundness
//! issues and there is no ETA for availability in Rust stable.
//!
//! The [`VisitorDispatch`] trait and its corresponding `#[derive(VisitorDispatch)]`
//! macro solve this problem.
//!
//! ## The VisitorDispatch trait
//!
//! The [`Visitable`] trait is implemented for all AST node types defined by the
//! `sqlparser` crate. The [`accept`](`Visitable::accept`) method requires an
//! implementation of [`VisitorDispatch`] as an argument.
//!
//! `VisitorDispatch` implementations must be able to handle any `sqlparser` AST
//! node type - but there are 200+ different node types in `sqlparser` and
//! manually defining 199 no-op `Visitor` implementations just so one specific
//! AST node type can be visited would a sub-optimal user experience.
//!
//! Hence the need for a smarter dispatch mechanism that can automatically "fill
//! in the gaps" and handle the node types that the user-defined type do not.
//!
//! `VisitorDispatch` can be implemented directly by a consumer of this crate,
//! however, the recommended approach is to implement [`Visitor`] on your
//! user-defined type for just the AST node types of interest and then derive
//! `VisitorDispatch`.
//!
//! # The FallbackVisitor trait
//!
//! This trait can be optionally implemented for a user-defined type. If
//! implemented, its `fallback_enter` and `fallback_exit` methods will be
//! invoked for AST nodes that are not handled by any of the `Visitor`
//! implementations defined for the user-defined type.
//!
#![doc = mermaid!("dispatch.mermaid")]

use simple_mermaid::mermaid;

use crate::{Enter, EnterControlFlow, Exit, ExitControlFlow, SqlNode, Visitable, Visitor};

pub use specialization::*;

/// Defines a simplistic trait specialization mechanism (in lieu of real
/// specialization landing in stable Rust)
pub mod specialization {
    use super::*;

    /// Type used by the `VisitorDispatch` derive macro in order to avoid
    /// conflicts with user-defined blanket implementations of traits defined
    /// in this crate.
    ///
    /// It is not anticipated that use of this type will be required outside of
    /// derived `VisitorDispatch` implementations and an understanding of its
    /// correct usage is therefore unnecessary. You may stop reading now :)
    ///
    /// `Dispatch` has its own blanket implementations of [`ViaVisitor`],
    /// [`ViaFallback`] and [`ViaIgnored`].
    ///
    /// The dispatch mechanism is the same as described in this blog post:
    /// [Generalized Autoref-Based Specialization](http://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html)
    ///
    /// `&mut &mut &mut Dispatch(user_type, node_type).enter(...)` (or `exit(...)`) will
    /// statically resolve to a call to one of the following in this order of
    /// presidence:
    ///
    /// 1. `ViaVisitor`
    /// 2. `ViaFallback`
    /// 3. `ViaIgnored`
    ///
    /// The following conditions must be true in order for the mechanism to work:
    ///
    /// - The type arguments `T` & `N` to `Dispatch` must be concrete at the
    ///   call-site
    ///
    /// - The call must not use less than three levels of `&mut` indirection.
    ///
    /// - The method signatures of `ViaVisitor`, `ViaFallback` and
    ///   `ViaIgnored` must be identical
    pub struct Dispatch<'ast, T, N>(pub T, pub &'ast N);

    /// Trait that is implemented for all `Dispatch<T, Node>` where `T:
    /// Visitor<Node>`.
    pub trait ViaVisitor<'ast, Node: Visitable<'ast>> {
        fn enter(&mut self, node: &'ast Node) -> EnterControlFlow;

        fn exit(&mut self, node: &'ast Node) -> ExitControlFlow;
    }

    /// Trait that is implemented for all types `Dispatch<T, Node>` where `T:
    /// FallbackVisitor<Node>`. Only invoked when `Dispatch<T, Node>` does
    /// **not** implement `Visitor<T, Node>.`
    pub trait ViaFallback<'ast, Node: Visitable<'ast>> {
        fn enter(&mut self, node: &'ast Node) -> EnterControlFlow;

        fn exit(&mut self, node: &'ast Node) -> ExitControlFlow;
    }

    /// Trait with a default blanket implementation for `Dispatch<T, Node>` where
    /// `T` implements neither `Visitor<Node>` nor `FallbackVisitor< Node>`.
    pub trait ViaIgnored<'ast, Node: Visitable<'ast>> {
        fn enter(&mut self, node: &'ast Node) -> EnterControlFlow;

        fn exit(&mut self, node: &'ast Node) -> ExitControlFlow;
    }

    impl<'ast, N: Visitable<'ast>, T> ViaVisitor<'ast, N> for &mut &mut Dispatch<'ast, &mut T, N>
    where
        T: Visitor<'ast, N>,
    {
        fn enter(&mut self, node: &'ast N) -> EnterControlFlow {
            Visitor::enter(self.0, node)
        }

        fn exit(&mut self, node: &'ast N) -> ExitControlFlow {
            Visitor::exit(self.0, node)
        }
    }

    impl<'ast, N: 'ast + Visitable<'ast>, T> ViaFallback<'ast, N> for &mut Dispatch<'ast, &mut T, N>
    where
        T: FallbackVisitor<'ast>,
        &'ast N: Into<SqlNode<'ast>>,
    {
        fn enter(&mut self, node: &'ast N) -> EnterControlFlow {
            FallbackVisitor::fallback_enter(self.0, node.into())
        }

        fn exit(&mut self, node: &'ast N) -> ExitControlFlow {
            FallbackVisitor::fallback_exit(self.0, node.into())
        }
    }

    impl<'ast, N: 'ast + Visitable<'ast>, T> ViaIgnored<'ast, N> for Dispatch<'ast, &mut T, N> {
        fn enter(&mut self, _: &'ast N) -> EnterControlFlow {
            Enter::visit()
        }

        fn exit(&mut self, _: &'ast N) -> ExitControlFlow {
            Exit::normal()
        }
    }
}

/// Implementations of this trait know how to dispatch any AST node type to
/// a [`Visitor`] impl, a [`FallbackVisitor`] impl or handle with a no-op
/// (in that order of precedence).
///
/// An implementation of this trait is required as an argument to
/// [`Visitable::accept`].
///
/// This trait is typically derived by using `#[derive(VisitorDispatch)]` on a
/// user-defined type but can be implemented directly.
#[allow(unused_variables)]
pub trait VisitorDispatch<'ast> {
    fn enter(&mut self, node: SqlNode<'ast>) -> EnterControlFlow;

    fn exit(&mut self, node: SqlNode<'ast>) -> ExitControlFlow;
}

/// Implementations of this trait will be invoked when `Self` does not implement
/// `Visitor<'ast, N>` for the wrapped node.
///
/// This can be useful for debugging and general sanity checks when trying to
/// verify the correctness of your [`Visitor`] implementations.
pub trait FallbackVisitor<'ast> {
    fn fallback_enter(&mut self, node: SqlNode<'ast>) -> EnterControlFlow;

    fn fallback_exit(&mut self, node: SqlNode<'ast>) -> ExitControlFlow;
}
