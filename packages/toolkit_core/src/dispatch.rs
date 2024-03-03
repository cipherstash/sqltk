//! # Dispatch
//!
//! This module defines the fallback trait implementation mechanism that makes
//! `#[derive(VisitorDispatch)]` work.
//!
//! When implementing a [`Visitor`] for a user-defined type it is not necessary
//! to manually implement `Visitor` for _all_ `sqlparser` AST node types.
//!
//! Rust makes this extraordinarily difficult to achieve because there is no
//! language capability in stable Rust that allows one to define a
//! default-yet-overidable blanket implementation of a trait.
//!
//! If/when impl specialization lands in the Rust compiler, it will be possible
//! to define overridable blanket implementations of traits which will make this
//! module redundant. There is no published timeline of when impl specialization
//! will land, so for now this workaround is necessary.
//!
//! ## VisitorDispatch
//!
//! The [`Visitable`] trait is implemented for all AST node types defined by the
//! `sqlparser` crate. The [`accept`](`Visitable::accept`) method requires an
//! implementation of [`VisitorDispatch`] as an argument.
//!
//! `VisitorDispatch` implementations must be able to handle any `sqlparser` AST
//! node type - but there are 200+ different node types in `sqlparser` and
//! manually defining 199 no-op `Visitor` implementations just so one specific
//! AST node type can be visited is a sub-optimal user experience.
//!
//! Hence the need for a smarter dispatch mechanism that can automatically "fill
//! in the gaps" and handle the node types that the user-defined type does not.
//!
//! `VisitorDispatch` can be implemented directly by a consumer of this crate,
//! however, the recommended approach is to implement [`Visitor`] on your
//! user-defined type for just the AST node types of interest and then derive
//! `VisitorDispatch`.
//!
//! Deriving `VisitorDispatch` works not by directly generating an
//! implementation of it, but by generating an implementation of
//! [`DispatchTable`] which is then picked up by a blanket implementation of
//! `VisitorDispatch` for all types that implement `DispatchTable`.
//!
#![doc = mermaid!("dispatch_handled.mermaid")]
#![doc = mermaid!("dispatch_fallback.mermaid")]

use simple_mermaid::mermaid;
use std::{marker::PhantomData, ops::ControlFlow};

use crate::{
    match_sql_node_enum, BoxOf, EnterControlFlow, ExitControlFlow, Navigation, OptionOf, SqlNode,
    VecOf, Visitable, Visitor,
};

/// Implementations of this trait know how to dispatch any AST node type from
/// `sqlparser` to a `Visitor<'_, N>` implementation, or invoke a fallback (no-op)
/// implementation.
///
/// An implementation of this trait is required as an argument to
/// [`Visitable::accept`].
///
/// This trait is typically derived by using `#[derive(VisitorDispatch)]` on a
/// user-defined type but can be implemented directly.
pub trait VisitorDispatch {
    fn enter<'subj, 'ast>(&'subj mut self, node: SqlNode<'ast>) -> EnterControlFlow
    where
        'ast: 'subj;
    fn exit<'subj, 'ast>(&'subj mut self, node: SqlNode<'ast>) -> ExitControlFlow
    where
        'ast: 'subj;
}

/// Marker type used by the [`DispatchTable`] trait to indicate that a
/// user-defined type does not implement `Visitor` for a specific AST node type
/// and should behave as a no-op instead.
///
/// This type is public due to being used in generated code but should not be used
/// directly.
pub struct Fallback<V>(PhantomData<V>);

/// Marker type used by the [`DispatchTable`] trait to indicate that a
/// user-defined type does implement `Visitor` for a specific AST node type.
///
/// This type is public due to being used in generated code but should not be used
/// directly.
pub struct Handle<V, N>(PhantomData<(V, N)>);

/// This trait acts like `Visitor<'_, N>` where `Self: Visitor<'_, N>` else
/// it will behave like a no-op fallback implementation.
///
/// This trait should never be implemented directly; there are blanket
/// implementations for [`Fallback`] and [`Handle`] which are applicable when
/// [`VisitorDispatch`] is derived for a user-defined type.
pub trait WithFallbackSupport<N>
where
    N: Visitable,
{
    /// The user-defined type (with #[derive(VisitorDispatch)])
    type Subject;

    /// Invoked when entering a `sqlparser` AST node.
    fn enter<'subj, 'ast>(
        maybe_visitor_of_node_type: &'subj mut Self::Subject,
        node: &'ast N,
    ) -> EnterControlFlow
    where
        'ast: 'subj;

    /// Invoked when exiting a `sqlparser` AST node.
    fn exit<'subj, 'ast>(
        maybe_visitor_of_node_type: &'subj mut Self::Subject,
        node: &'ast N,
    ) -> ExitControlFlow
    where
        'ast: 'subj;
}

/// Implementation for user-defined types that derive [`VisitorDispatch`] and
/// implement [`Visitor<N>`].
///
/// [`WithFallbackSupport::enter`] and [`WithFallbackSupport::exit`] forward
/// to the user-defined `Visitor` implementation methods.
impl<N, V> WithFallbackSupport<N> for Handle<V, N>
where
    N: Visitable,
    V: Visitor<N>,
{
    type Subject = V;

    fn enter<'subj, 'ast>(visitor: &'subj mut Self::Subject, node: &'ast N) -> EnterControlFlow
    where
        'ast: 'subj,
    {
        V::enter(visitor, node)
    }

    fn exit<'subj, 'ast>(visitor: &'subj mut Self::Subject, node: &'ast N) -> ExitControlFlow
    where
        'ast: 'subj,
    {
        V::exit(visitor, node)
    }
}

/// Implementation for user-defined types that derive [`VisitorDispatch`] and
/// **do not** implement [`Visitor<'ast, N>`], and therefore acts as a fallback
/// implementation.
///
/// [`WithFallbackSupport::enter`] returns `ControlFlow::Continue(Navigation::Visit)`.
/// [`WithFallbackSupport::exit`] returns `ControlFlow::Continue(())`.
impl<N, V> WithFallbackSupport<N> for Fallback<V>
where
    N: Visitable,
{
    type Subject = V;

    fn enter<'subj, 'ast>(_: &'subj mut Self::Subject, _: &'ast N) -> EnterControlFlow
    where
        'ast: 'subj,
    {
        ControlFlow::Continue(Navigation::Visit)
    }

    fn exit<'subj, 'ast>(_: &'subj mut Self::Subject, _: &'ast N) -> ExitControlFlow
    where
        'ast: 'subj,
    {
        ControlFlow::Continue(())
    }
}

/// A type that only exists at compile time that is used to pick one of two types
/// based on the truthiness of a boolean constant.
pub struct Cond<const COND: bool, Then, Else>(PhantomData<(Then, Else)>);

/// Helper trait for resolving a [`Cond`] whose `Output` associated type will
/// be one of the `Then` or `Else` generic arguments to the `Cond`.
pub trait ResolveCond {
    type Output;
}

/// Resolves `Cond<true, Then, Else>` as `Then`.
impl<Then, Else> ResolveCond for Cond<true, Then, Else> {
    type Output = Then;
}

/// Resolves `Cond<false, Then, Else>` as `Else`.
impl<Then, Else> ResolveCond for Cond<false, Then, Else> {
    type Output = Else;
}

/// Helper type alias for use in derived [`DispatchTable`] implementations.
pub type If<const COND: bool, Then, Else> = <Cond<{ COND }, Then, Else> as ResolveCond>::Output;

/// Helper trait to assist in compile-time detection of whether a specific type
/// implements a specific trait.
pub trait Nope {
    const ANSWER: bool = false;
}

/// Every type implements `Nope`.
impl<V> Nope for V {}

/// Compile-time helper struct used for detecting at compile time if a
/// user-defined type implements `Visitor<'_, N>`.
pub struct IsVisitor<V, N>(PhantomData<(V, N)>);

/// Sole implementation of `IsVisitor`. Only applies when the user-defined type
/// does implement `Visitor<'_, N>` and shadows the value defined by the blanket
/// implementation of [`Nope`].
///
/// Credit where credit is due: this technique was lifted from the `impls` crate.
impl<V, N> IsVisitor<V, N>
where
    N: Visitable,
    V: Visitor<N>,
{
    pub const ANSWER: bool = true;
}

/// Trait that is implemented for all `sqlparser` AST nodes.
///
/// The associated type allows the dispatch fallback mechanism to determine at
/// compile time whether a particular user-defined type implements `Visitor<'_,
/// Self>` or whether it needs a fallback.
pub trait DispatchTableLookup
where
    Self: Sized,
    Self: Visitable,
{
    type Lookup<Table: DispatchTable>: WithFallbackSupport<Self>;
}

/// A trait dispatching a specific AST node type.
///
/// Users should not implement this trait directly; a blanket implementation
/// applies for all types that derive [`VisitorDispatch`].
pub trait VisitorDispatchNode<N>
where
    N: Visitable,
{
    fn enter<'subj, 'ast>(&'subj mut self, node: &'ast N) -> EnterControlFlow
    where
        'ast: 'subj;

    fn exit<'subj, 'ast>(&'subj mut self, node: &'ast N) -> ExitControlFlow
    where
        'ast: 'subj;
}

/// Blanket implementation for all types `V` that implement [`DispatchTable`].
/// Every used-defined type that derives [`VisitorDispatch`] also has a derived
/// `DispatchTable` implementation.
impl<V, N> VisitorDispatchNode<N> for V
where
    V: DispatchTable,
    N: Visitable,
    N: DispatchTableLookup,
    N::Lookup<V>: WithFallbackSupport<N, Subject = Self>,
{
    fn enter<'subj, 'ast>(&'subj mut self, node: &'ast N) -> EnterControlFlow
    where
        'ast: 'subj,
    {
        <N::Lookup<V> as WithFallbackSupport<N>>::enter(self, node)
    }

    fn exit<'subj, 'ast>(&'subj mut self, node: &'ast N) -> ExitControlFlow
    where
        'ast: 'subj,
    {
        <N::Lookup<V> as WithFallbackSupport<N>>::exit(self, node)
    }
}

include!(concat!(
    env!("OUT_DIR"),
    "/generated_dispatch_table_trait.rs"
));

include!(concat!(
    env!("OUT_DIR"),
    "/generated_dispatch_table_lookup_impls.rs"
));

include!(concat!(env!("OUT_DIR"), "/generated_dispatch_impls.rs"));
