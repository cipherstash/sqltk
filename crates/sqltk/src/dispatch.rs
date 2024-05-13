use core::{any::Any, fmt::Debug, marker::PhantomData};
use std::error::Error;

use crate::{flow, Node, SpecializedVisitor, Visitor, VisitorControlFlow};

pub fn generalise<'ast, V, N: 'static, State, E>(visitor: V) -> impl Visitor<'ast, State, E>
where
    E: Error + Debug,
    V: SpecializedVisitor<'ast, N, State, E>,
{
    GeneralVisitor {
        inner: visitor,
        _target: Default::default(),
        _state: Default::default(),
        _error: Default::default(),
    }
}

pub struct GeneralVisitor<'ast, TargetNode, State, E, V> {
    inner: V,
    _target: PhantomData<&'ast TargetNode>,
    _state: PhantomData<State>,
    _error: PhantomData<E>,
}

impl<'ast, TargetNode, State, E, V> Visitor<'ast, State, E>
    for GeneralVisitor<'ast, TargetNode, State, E, V>
where
    TargetNode: 'static,
    E: Error + Debug,
    V: SpecializedVisitor<'ast, TargetNode, State, E>,
{
    fn enter<N: 'static>(&self, node: &'ast N, state: State) -> VisitorControlFlow<'ast, State, E> {
        if let Some(target) = (node as &dyn Any).downcast_ref::<TargetNode>() {
            SpecializedVisitor::enter(&self.inner, target, state)
        } else {
            flow::cont(state)
        }
    }

    fn exit<N: 'static>(&self, node: &'ast N, state: State) -> VisitorControlFlow<'ast, State, E> {
        if let Some(target) = (node as &dyn Any).downcast_ref::<TargetNode>() {
            SpecializedVisitor::exit(&self.inner, target, state)
        } else {
            flow::cont(state)
        }
    }
}

/// Dispatches to specific AST node types.
///
/// This type is used by generated `Visitable` implementations and is not
/// intended for direct use.
#[doc(hidden)]
pub struct DowncastDispatcher<'ast, DowncastTo, State, FnEnter, FnExit> {
    enter_fn: FnEnter,
    exit_fn: FnExit,
    _phantom: PhantomData<&'ast (DowncastTo, State)>,
}

impl<'ast, DowncastTo, State, FnEnter, FnExit>
    DowncastDispatcher<'ast, DowncastTo, State, FnEnter, FnExit>
{
    pub fn new(enter_fn: FnEnter, exit_fn: FnExit) -> Self {
        Self {
            enter_fn,
            exit_fn,
            _phantom: PhantomData,
        }
    }
}

impl<'ast, DowncastTo, State, E, FnEnter, FnExit> Visitor<'ast, State, E>
    for DowncastDispatcher<'ast, DowncastTo, State, FnEnter, FnExit>
where
    DowncastTo: 'static,
    FnEnter: Fn(&'ast DowncastTo, State) -> VisitorControlFlow<'ast, State, E>,
    FnExit: Fn(&'ast DowncastTo, State) -> VisitorControlFlow<'ast, State, E>,
{
    fn enter<N: 'static>(&self, node: &'ast N, state: State) -> VisitorControlFlow<'ast, State, E> {
        match (node as &dyn Any).downcast_ref::<DowncastTo>() {
            Some(node) => (self.enter_fn)(node, state),
            None => flow::cont(state),
        }
    }

    fn exit<N: 'static>(&self, node: &'ast N, state: State) -> VisitorControlFlow<'ast, State, E> {
        match (node as &dyn Any).downcast_ref::<DowncastTo>() {
            Some(node) => (self.exit_fn)(node, state),
            None => flow::cont(state),
        }
    }
}

///
pub(crate) fn ignore<'ast, State, E, N>(
) -> impl Fn(&'ast N, State) -> VisitorControlFlow<'ast, State, E> {
    |_: &'ast N, state: State| flow::cont(state)
}

/// Module of [`Visitor`]-building functions that will match any AST nodes
/// irrespective of their type.
#[allow(non_snake_case)]
pub mod AnyNode {
    use super::*;

    /// Returns a [`Visitor`] impl whose `enter` method will invoke
    /// `enter_fn` for nodes of any type.
    pub fn on_enter<'ast, State: 'ast, E, FnEnter>(
        enter_fn: FnEnter,
    ) -> impl Visitor<'ast, State, E>
    where
        FnEnter: Fn(Node<'ast>, State) -> VisitorControlFlow<'ast, State, E>,
    {
        AnyNode::on_enter_exit(enter_fn, AnyNode::ignore::<'ast, State, E>())
    }

    /// Returns a [`Visitor`] impl whose `exit` method will invoke
    /// `exit_fn` for nodes of any type.
    pub fn on_exit<'ast, State: 'ast, E, FnExit>(exit_fn: FnExit) -> impl Visitor<'ast, State, E>
    where
        FnExit: Fn(Node<'ast>, State) -> VisitorControlFlow<'ast, State, E>,
    {
        AnyNode::on_enter_exit(AnyNode::ignore::<'ast, State, E>(), exit_fn)
    }

    /// Returns a [`Visitor`] impl whose `enter` & `exit` methods will
    /// invoke `enter_fn` & `exit_fn` for nodes of any type.
    pub fn on_enter_exit<'ast, State: 'ast, E, FnEnter, FnExit>(
        enter_fn: FnEnter,
        exit_fn: FnExit,
    ) -> impl Visitor<'ast, State, E>
    where
        FnEnter: Fn(Node<'ast>, State) -> VisitorControlFlow<'ast, State, E>,
        FnExit: Fn(Node<'ast>, State) -> VisitorControlFlow<'ast, State, E>,
    {
        AnyNodeDispatcher::<'ast, State, _, _>::new(enter_fn, exit_fn)
    }

    fn ignore<'ast, State: 'ast, E>() -> impl Fn(Node, State) -> VisitorControlFlow<'ast, State, E>
    {
        |_: Node, state: State| flow::cont(state)
    }
}

struct AnyNodeDispatcher<'ast, State, FnEnter, FnExit> {
    enter_fn: FnEnter,
    exit_fn: FnExit,
    _phantom: PhantomData<&'ast State>,
}

impl<'ast, State, FnEnter, FnExit> AnyNodeDispatcher<'ast, State, FnEnter, FnExit> {
    pub fn new(enter_fn: FnEnter, exit_fn: FnExit) -> Self {
        Self {
            enter_fn,
            exit_fn,
            _phantom: PhantomData,
        }
    }
}

impl<'ast, State, E, FnEnter, FnExit> Visitor<'ast, State, E>
    for AnyNodeDispatcher<'ast, State, FnEnter, FnExit>
where
    FnEnter: Fn(Node<'ast>, State) -> VisitorControlFlow<'ast, State, E>,
    FnExit: Fn(Node<'ast>, State) -> VisitorControlFlow<'ast, State, E>,
{
    fn enter<N: 'static>(&self, node: &'ast N, state: State) -> VisitorControlFlow<'ast, State, E>
    where
        &'ast N: Into<Node<'ast>>,
    {
        (self.enter_fn)(node.into(), state)
    }

    fn exit<N: 'static>(&self, node: &'ast N, state: State) -> VisitorControlFlow<'ast, State, E>
    where
        &'ast N: Into<Node<'ast>>,
    {
        (self.exit_fn)(node.into(), state)
    }
}
