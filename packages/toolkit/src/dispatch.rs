use std::{marker::PhantomData, ops::ControlFlow};

use crate::{AstNode, ConcreteNode, EnterControlFlow, ExitControlFlow, Navigation, Node, Visitor};

pub struct Fallback<T>(PhantomData<T>);

pub struct Handle<V, N>(PhantomData<(V, N)>);

pub trait WithFallbackSupport<'ast, N: 'ast + AstNode<'ast>> {
    type Subject;

    fn enter(maybe_visitor: &mut Self::Subject, node: Node<'ast, N>) -> EnterControlFlow;

    fn exit(maybe_visitor: &mut Self::Subject, node: Node<'ast, N>) -> ExitControlFlow;
}

impl<'ast, N: 'ast + AstNode<'ast>, V> WithFallbackSupport<'ast, N> for Handle<V, N>
where
    V: Visitor<'ast, N>,
{
    type Subject = V;

    fn enter(visitor: &mut Self::Subject, node: Node<'ast, N>) -> EnterControlFlow {
        V::enter(visitor, node)
    }

    fn exit(visitor: &mut Self::Subject, node: Node<'ast, N>) -> ExitControlFlow {
        V::exit(visitor, node)
    }
}

impl<'ast, N: 'ast + AstNode<'ast>, V> WithFallbackSupport<'ast, N> for Fallback<V> {
    type Subject = V;

    fn enter(_: &mut Self::Subject, _: Node<'ast, N>) -> EnterControlFlow {
        ControlFlow::Continue(Navigation::Visit)
    }

    fn exit(_: &mut Self::Subject, _: Node<'ast, N>) -> ExitControlFlow {
        ControlFlow::Continue(())
    }
}

pub struct Cond<const COND: bool, Then, Else>(PhantomData<(Then, Else)>);

pub trait Resolve {
    type Output;
}

impl<Then, Else> Resolve for Cond<true, Then, Else> {
    type Output = Then;
}

impl<Then, Else> Resolve for Cond<false, Then, Else> {
    type Output = Else;
}

pub type If<const COND: bool, Then, Else> = <Cond<{ COND }, Then, Else> as Resolve>::Output;

pub trait AssumeNotImplemented {
    const ANSWER: bool = false;
}

impl<V> AssumeNotImplemented for V {}

pub struct Visits<V, N>(core::marker::PhantomData<(V, N)>);

impl<V, N: AstNode<'static>> Visits<V, N>
where
    V: Visitor<'static, N>,
{
    pub const ANSWER: bool = true;
}

pub trait DispatchTableLookup<'ast>
where
    Self: Sized + AstNode<'ast>,
{
    type Lookup<Table: DispatchTable<'ast>>: WithFallbackSupport<'ast, Self>;
}

pub trait VisitorDispatch<'ast> {
    fn enter(&mut self, node: ConcreteNode<'ast>) -> EnterControlFlow;
    fn exit(&mut self, node: ConcreteNode<'ast>) -> ExitControlFlow;
}

pub trait VisitorDispatchNode<'ast, N: AstNode<'ast>> {
    fn enter(&mut self, node: Node<'ast, N>) -> EnterControlFlow;
    fn exit(&mut self, node: Node<'ast, N>) -> ExitControlFlow;
}

impl<'ast, V, N> VisitorDispatchNode<'ast, N> for V
where
    V: DispatchTable<'ast>,
    N: DispatchTableLookup<'ast>,
    N::Lookup<V>: WithFallbackSupport<'ast, N, Subject = Self>,
{
    fn enter(&mut self, node: Node<'ast, N>) -> EnterControlFlow {
        <N::Lookup<V> as WithFallbackSupport<N>>::enter(self, node)
    }

    fn exit(&mut self, node: Node<'ast, N>) -> ExitControlFlow {
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
