use crate::*;

impl<'ast, T: AstNode<'ast>> AstNode<'ast> for &'ast T {
    fn accept_with_node_builder<V>(
        &'ast self,
        visitor: &mut V,
        node_builder: &mut NodeBuilder,
    ) -> VisitorControlFlow
    where
        V: VisitorDispatch<'ast>,
    {
        (*self).accept_with_node_builder(visitor, node_builder)
    }
}

impl<'ast, T: AstNode<'ast>> AstNode<'ast> for Vec<T>
where
    Node<'ast, Vec<T>>: Into<ConcreteNode<'ast>>,
{
    fn accept_with_node_builder<V>(
        &'ast self,
        visitor: &mut V,
        node_builder: &mut NodeBuilder,
    ) -> VisitorControlFlow
    where
        V: VisitorDispatch<'ast>,
    {
        if self.is_empty() {
            nav_skip()
        } else {
            visit(node_builder.new_node(self).into(), visitor, |visitor| {
                for child in self.iter() {
                    child.accept_with_node_builder(visitor, node_builder)?;
                }
                nav_visit()
            })
        }
    }
}

impl<'ast, T: AstNode<'ast>> AstNode<'ast> for Box<T>
where
    Node<'ast, Box<T>>: Into<ConcreteNode<'ast>>,
{
    fn accept_with_node_builder<V: VisitorDispatch<'ast>>(
        &'ast self,
        visitor: &mut V,
        node_builder: &mut NodeBuilder,
    ) -> VisitorControlFlow {
        visit(node_builder.new_node(self).into(), visitor, |visitor| {
            (**self).accept_with_node_builder(visitor, node_builder)
        })
    }
}

impl<'ast, T: AstNode<'ast>> AstNode<'ast> for Option<T>
where
    Node<'ast, Option<T>>: Into<ConcreteNode<'ast>>,
{
    fn accept_with_node_builder<V: VisitorDispatch<'ast>>(
        &'ast self,
        visitor: &mut V,
        node_builder: &mut NodeBuilder,
    ) -> VisitorControlFlow {
        if self.is_none() {
            nav_skip()
        } else {
            visit(
                node_builder.new_node(self).into(),
                visitor,
                |visitor| match self {
                    Some(child) => child.accept_with_node_builder(visitor, node_builder),
                    None => nav_skip(),
                },
            )
        }
    }
}

