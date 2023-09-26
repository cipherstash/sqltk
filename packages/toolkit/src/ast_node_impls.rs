use crate::{*};

impl<'ast, T: AstNode<'ast>> AstNode<'ast> for &'ast T {
    fn accept_with_id_iter<V>(
        &'ast self,
        visitor: &V,
        node_builder: &mut NodeBuilder,
    ) -> VisitorControlFlow where V: VisitorDispatch<'ast> {
        (*self).accept_with_id_iter(visitor, node_builder)
    }
}

impl<'ast, T: AstNode<'ast>> AstNode<'ast> for Vec<T> where Node<'ast, Vec<T>>: Into<ConcreteNode<'ast>> {
    fn accept_with_id_iter<V>(
        &'ast self,
        visitor: &V,
        node_builder: &mut NodeBuilder,
    ) -> VisitorControlFlow where V: VisitorDispatch<'ast> {
        visit(node_builder.new_node(self).into(), visitor, move || {
            for child in self.iter() {
                visit(node_builder.new_node(self).into(), visitor, || {
                    child.accept_with_id_iter(visitor, node_builder)
                })?;
            }
            nav_skip()
        })
    }
}

impl<'ast, T: AstNode<'ast>> AstNode<'ast> for Box<T> where Node<'ast, Box<T>>: Into<ConcreteNode<'ast>> {
    fn accept_with_id_iter<V: VisitorDispatch<'ast>>(
        &'ast self,
        visitor: &V,
        node_builder: &mut NodeBuilder,
    ) -> VisitorControlFlow {
        visit(node_builder.new_node(self).into(), visitor, || {
            (&**self).accept_with_id_iter(visitor, node_builder)
        })
    }
}

impl<'ast, T: AstNode<'ast>> AstNode<'ast> for Option<T> where Node<'ast, Option<T>>: Into<ConcreteNode<'ast>> {
    fn accept_with_id_iter<V: VisitorDispatch<'ast>>(
        &'ast self,
        visitor: &V,
        node_builder: &mut NodeBuilder,
    ) -> VisitorControlFlow {
        visit(node_builder.new_node(self).into(), visitor, || match self {
            Some(child) => child.accept_with_id_iter(visitor, node_builder),
            None => nav_skip(),
        })
    }
}

include!(concat!(env!("OUT_DIR"), "/generated_ast_node_impls.rs"));
