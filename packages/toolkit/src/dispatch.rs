use crate::{
    match_concrete_node, AstNode, BoxOf, ConcreteNode, Node, OptionOf, VecOf, Visitor,
    VisitorControlFlow,
};

pub struct FallbackVisitor;

/// Global `FallbackVisitor` instance.
///
/// The `mut` is necessary in order to conform to the various traits that
/// accept `&mut self`, but due to `FallbackVisitor` being stateless this is
/// 100% safe.
pub static mut FALLBACK_VISITOR: FallbackVisitor = FallbackVisitor;

include!(concat!(env!("OUT_DIR"), "/generated_dispatch_impls.rs"));

pub trait VisitorDispatchNode<'ast, N: AstNode<'ast>>
where
    Self: NodeSupport<N>,
{
    fn dispatch_node_enter(&mut self, node: Node<'ast, N>) -> VisitorControlFlow;

    fn dispatch_node_exit(&mut self, node: Node<'ast, N>) -> VisitorControlFlow;
}

impl<'ast, N: AstNode<'ast>, V> VisitorDispatchNode<'ast, N> for V
where
    N: 'ast,
    Self: NodeSupport<N> + MaybeFallback<'ast, N, <Self as NodeSupport<N>>::Supported>,
{
    fn dispatch_node_enter(&mut self, node: Node<'ast, N>) -> VisitorControlFlow {
        self.visitor().enter(node)
    }

    fn dispatch_node_exit(&mut self, node: Node<'ast, N>) -> VisitorControlFlow {
        self.visitor().exit(node)
    }
}

pub trait NodeSupport<Node: ?Sized> {
    type Supported;
}

impl<'ast, N: AstNode<'ast>> Visitor<'ast, N> for FallbackVisitor {}

pub trait MaybeFallback<'ast, N: AstNode<'ast>, Supported> {
    type Visitor: Visitor<'ast, N>;
    fn visitor(&mut self) -> &mut Self::Visitor;
}

pub struct Condition<const BOOL: bool>;

impl<'ast, N, V> MaybeFallback<'ast, N, Condition<true>> for V
where
    N: AstNode<'ast>,
    V: NodeSupport<N, Supported = Condition<true>> + Visitor<'ast, N>,
{
    type Visitor = Self;

    fn visitor(&mut self) -> &mut Self::Visitor {
        self
    }
}

impl<'ast, N, V> MaybeFallback<'ast, N, Condition<false>> for V
where
    N: AstNode<'ast>,
    V: NodeSupport<N, Supported = Condition<false>>,
{
    type Visitor = FallbackVisitor;

    fn visitor(&mut self) -> &mut Self::Visitor {
        // SAFETY: `FallbackVisitor` has no state so this is actually safe
        // A new instance of the visitor cannot be created here because it
        // is returned by reference.
        unsafe { &mut FALLBACK_VISITOR }
    }
}
