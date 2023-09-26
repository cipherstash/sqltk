use crate::{AstNode, Node, Visitor, VisitorControlFlow};

pub struct FallbackVisitor;

pub const FALLBACK_VISITOR: &'static FallbackVisitor = &FallbackVisitor;

include!(concat!(env!("OUT_DIR"), "/generated_concrete_node_enum.rs"));
include!(concat!(env!("OUT_DIR"), "/generated_concrete_node_enum_match_macro.rs"));
include!(concat!(env!("OUT_DIR"), "/generated_dispatch_impls.rs"));

pub trait VisitorDispatchNode<'ast, N: AstNode<'ast>>
where
    Self: NodeSupport<N>,
{
    fn dispatch_node_enter(&self, node: Node<'ast, N>) -> VisitorControlFlow;

    fn dispatch_node_exit(&self, node: Node<'ast, N>) -> VisitorControlFlow;
}

impl<'ast, N: AstNode<'ast>, V> VisitorDispatchNode<'ast, N> for V
where
    N: 'ast,
    Self: NodeSupport<N> + MaybeFallback<'ast, N, <Self as NodeSupport<N>>::Supported>,
{
    fn dispatch_node_enter(&self, node: Node<'ast, N>) -> VisitorControlFlow {
        self.visitor().enter(node)
    }

    fn dispatch_node_exit(&self, node: Node<'ast, N>) -> VisitorControlFlow {
        self.visitor().exit(node)
    }
}

pub trait NodeSupport<Node: ?Sized> {
    type Supported;
}

impl<'ast, N: AstNode<'ast>> Visitor<'ast, N> for FallbackVisitor {}

pub trait MaybeFallback<'ast, N: AstNode<'ast>, Supported> {
    type Visitor: Visitor<'ast, N>;
    fn visitor(&self) -> &Self::Visitor;
}

pub struct Condition<const BOOL: bool>;

impl<'ast, N, V> MaybeFallback<'ast, N, Condition<true>> for V
where
    N: AstNode<'ast>,
    V: NodeSupport<N, Supported = Condition<true>> + Visitor<'ast, N>,
{
    type Visitor = Self;

    fn visitor(&self) -> &Self::Visitor {
        self
    }
}

impl<'ast, N, V> MaybeFallback<'ast, N, Condition<false>> for V
where
    N: AstNode<'ast>,
    V: NodeSupport<N, Supported = Condition<false>>,
{
    type Visitor = FallbackVisitor;

    fn visitor(&self) -> &Self::Visitor {
        &FallbackVisitor
    }
}
