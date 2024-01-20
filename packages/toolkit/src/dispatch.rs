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

impl<'ast, N: AstNode<'ast>, Target, V> VisitorDispatchNode<'ast, N> for V
where
    N: 'ast,
    Target: Visitor<'ast, N>,
    Self: NodeSupport<N>
        + MaybeFallback<'ast, N, <Self as NodeSupport<N>>::Supported, Target = Target>,
{
    fn dispatch_node_enter(&mut self, node: Node<'ast, N>) -> VisitorControlFlow {
        self.target().enter(node)
    }

    fn dispatch_node_exit(&mut self, node: Node<'ast, N>) -> VisitorControlFlow {
        self.target().exit(node)
    }
}

pub trait NodeSupport<Node: ?Sized> {
    type Supported;
}

impl<'ast, N: AstNode<'ast>> Visitor<'ast, N> for FallbackVisitor {}

pub trait MaybeFallback<'ast, N: AstNode<'ast>, Supported> {
    type Target;
    fn target(&mut self) -> &mut Self::Target;
}

pub struct Condition<const BOOL: bool>;

impl<'ast, N, V> MaybeFallback<'ast, N, Condition<true>> for V
where
    N: AstNode<'ast>,
    V: NodeSupport<N, Supported = Condition<true>> + Visitor<'ast, N>,
{
    type Target = Self;

    fn target(&mut self) -> &mut Self::Target {
        self
    }
}

impl<'ast, N, V> MaybeFallback<'ast, N, Condition<false>> for V
where
    N: AstNode<'ast>,
    V: NodeSupport<N, Supported = Condition<false>>,
{
    type Target = FallbackVisitor;

    fn target(&mut self) -> &mut Self::Target {
        // SAFETY: `FallbackVisitor` has no state so this is actually safe
        // A new instance of the visitor cannot be created here because it
        // is returned by reference.
        unsafe { &mut FALLBACK_VISITOR }
    }
}
