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

use crate::{Flow, SqlNode, Visitable, Visitor, VisitorControlFlow};

pub use specialization::*;

/// Defines a simplistic trait specialization mechanism (in lieu of real
/// specialization landing in stable Rust)
pub mod specialization {

    use std::marker::PhantomData;

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
    /// precedence:
    ///
    /// 1. `ViaVisitor`
    /// 2. `ViaFallback`
    /// 3. `ViaIgnored`
    ///
    /// The following conditions must be true in order for the mechanism to work:
    ///
    /// - The type arguments `Visitor`, `Node`, `State` to `Dispatch` must be
    ///   concrete at the call site.
    ///
    /// - The call must not use less than three levels of `&` indirection.
    ///
    /// - The method signatures of `ViaVisitor`, `ViaFallback` and
    ///   `ViaIgnored` must be identical.
    pub struct Dispatch<'visitor, 'ast, V, Node, State>(
        pub &'visitor V,
        pub &'ast Node,
        pub PhantomData<State>,
    );

    /// Forwards calls to a `Visitor` impl if the [`Dispatch`] target implements
    /// `Visitor<Node, State>`.
    pub trait ViaVisitor<'ast, Node, State>
    where
        Node: Visitable<'ast>,
    {
        fn enter(&self, node: &'ast Node, state: State) -> VisitorControlFlow<State>;

        fn exit(&self, node: &'ast Node, state: State) -> VisitorControlFlow<State>;
    }

    /// Forwards calls to a `FallbackVisitor` impl if the [`Dispatch`] target
    /// implements `FallbackVisitor<State>` and the `Dispatch` target *does not*
    /// implement `Visitor<Node, State>`.
    ///
    /// An example use case for a `FallbackVisitor` is to force an error when a
    /// node type is encountered that you are not expecting.
    pub trait ViaFallback<'ast, Node, State>
    where
        Node: Visitable<'ast>,
    {
        fn enter(&self, node: &'ast Node, state: State) -> VisitorControlFlow<State>;

        fn exit(&self, node: &'ast Node, state: State) -> VisitorControlFlow<State>;
    }

    /// When there is no applicable [`Visitor`] or [`FallbackVisitor`] implementation,
    /// calls to `enter` and `exit` are handled by a blanket implementation that returns
    /// `Flow::cont(state)`. This is non-overridable.
    pub trait ViaIgnored<'ast, Node, State>
    where
        Node: Visitable<'ast>,
    {
        fn enter(&self, node: &'ast Node, state: State) -> VisitorControlFlow<State>;

        fn exit(&self, node: &'ast Node, state: State) -> VisitorControlFlow<State>;
    }

    impl<'v, 'ast, N, State, V> ViaVisitor<'ast, N, State> for &&Dispatch<'v, 'ast, V, N, State>
    where
        N: Visitable<'ast>,
        V: Visitor<'ast, N, State>,
    {
        fn enter(&self, node: &'ast N, state: State) -> VisitorControlFlow<State> {
            Visitor::enter(self.0, node, state)
        }

        fn exit(&self, node: &'ast N, state: State) -> VisitorControlFlow<State> {
            Visitor::exit(self.0, node, state)
        }
    }

    impl<'v, 'ast, N, V, State> ViaFallback<'ast, N, State> for &Dispatch<'v, 'ast, V, N, State>
    where
        N: 'ast + Visitable<'ast>,
        &'v V: FallbackVisitor<'ast, State>,
        &'ast N: Into<SqlNode<'ast>>,
    {
        fn enter(&self, node: &'ast N, state: State) -> VisitorControlFlow<State> {
            FallbackVisitor::fallback_enter(&self.0, node.into(), state)
        }

        fn exit(&self, node: &'ast N, state: State) -> VisitorControlFlow<State> {
            FallbackVisitor::fallback_exit(&self.0, node.into(), state)
        }
    }

    impl<'v, 'ast, N, V, State> ViaIgnored<'ast, N, State> for Dispatch<'v, 'ast, V, N, State>
    where
        N: 'ast + Visitable<'ast>,
    {
        fn enter(&self, _: &'ast N, state: State) -> VisitorControlFlow<State> {
            Flow::cont(state)
        }

        fn exit(&self, _: &'ast N, state: State) -> VisitorControlFlow<State> {
            Flow::cont(state)
        }
    }

    // Due to the way that Rust infers the type arguments for the Dispatch
    // struct we need a blanket impementation of `&V: Visitor` where `V:
    // Visitor`.
    impl<'v, 'ast, N: Visitable<'ast>, V, State> Visitor<'ast, N, State> for &'v V
    where
        V: Visitor<'ast, N, State>,
    {
        fn enter(&self, node: &'ast N, state: State) -> VisitorControlFlow<State> {
            Visitor::enter(*self, node, state)
        }

        fn exit(&self, node: &'ast N, state: State) -> VisitorControlFlow<State> {
            Visitor::exit(*self, node, state)
        }
    }
}

/// This trait is typically derived by using `#[derive(VisitorDispatch)]` on a
/// user-defined type but can be implemented directly.
///
/// Derived implementations of this trait know how to dispatch any AST node type
/// to a [`Visitor`] impl, a [`FallbackVisitor`] impl or handle with a no-op (in
/// that order of precedence).
#[allow(unused_variables)]
pub trait VisitorDispatch<'ast, State> {
    fn enter(&self, node: SqlNode<'ast>, state: State) -> VisitorControlFlow<State>;

    fn exit(&self, node: SqlNode<'ast>, state: State) -> VisitorControlFlow<State>;
}

/// Implementations of this trait will be invoked when `Self` does not implement
/// `Visitor` for the wrapped node.
///
/// This can be useful for debugging and general sanity checks when trying to
/// verify the correctness of your [`Visitor`] implementations.
pub trait FallbackVisitor<'ast, State> {
    fn fallback_enter(&self, node: SqlNode<'ast>, state: State) -> VisitorControlFlow<State>;

    fn fallback_exit(&self, node: SqlNode<'ast>, state: State) -> VisitorControlFlow<State>;
}
