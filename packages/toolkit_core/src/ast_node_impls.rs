use crate::*;

impl<'ast, T: AstNode<'ast>> AstNode<'ast> for &'ast T {
    fn accept_and_identify<V>(
        &'ast self,
        visitor: &mut V,
        node_id_seq: &mut NodeIdSequence,
    ) -> EnterControlFlow
    where
        V: VisitorDispatch<'ast>,
    {
        (*self).accept_and_identify(visitor, node_id_seq)
    }
}

impl<'ast, T: AstNode<'ast>> AstNode<'ast> for Vec<T>
where
    Node<'ast, Vec<T>>: Into<SqlNode<'ast>>,
{
    fn accept_and_identify<V>(
        &'ast self,
        visitor: &mut V,
        node_id_seq: &mut NodeIdSequence,
    ) -> EnterControlFlow
    where
        V: VisitorDispatch<'ast>,
    {
        if self.is_empty() {
            ControlFlow::Continue(Navigation::Skip)
        } else {
            visit(node_id_seq.next_node(self).into(), visitor, |visitor| {
                for child in self.iter() {
                    child.accept_and_identify(visitor, node_id_seq)?;
                }
                ControlFlow::Continue(Navigation::Visit)
            })
        }
    }
}

impl<'ast, T: AstNode<'ast>> AstNode<'ast> for Box<T>
where
    Node<'ast, Box<T>>: Into<SqlNode<'ast>>,
{
    fn accept_and_identify<V: VisitorDispatch<'ast>>(
        &'ast self,
        visitor: &mut V,
        node_id_seq: &mut NodeIdSequence,
    ) -> EnterControlFlow {
        visit(node_id_seq.next_node(self).into(), visitor, |visitor| {
            (**self).accept_and_identify(visitor, node_id_seq)
        })
    }
}

impl<'ast, T: AstNode<'ast>> AstNode<'ast> for Option<T>
where
    Node<'ast, Option<T>>: Into<SqlNode<'ast>>,
{
    fn accept_and_identify<V: VisitorDispatch<'ast>>(
        &'ast self,
        visitor: &mut V,
        node_id_seq: &mut NodeIdSequence,
    ) -> EnterControlFlow {
        if self.is_none() {
            ControlFlow::Continue(Navigation::Skip)
        } else {
            visit(
                node_id_seq.next_node(self).into(),
                visitor,
                |visitor| match self {
                    Some(child) => child.accept_and_identify(visitor, node_id_seq),
                    None => ControlFlow::Continue(Navigation::Skip),
                },
            )
        }
    }
}
