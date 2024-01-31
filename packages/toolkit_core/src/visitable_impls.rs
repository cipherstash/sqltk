use crate::*;

impl<'ast, T: Visitable<'ast>> Visitable<'ast> for &'ast T {
    fn accept_and_identify(
        &'ast self,
        visitor: &mut dyn VisitorDispatch<'ast>,
        node_id_seq: &mut NodeIdSequence,
    ) -> EnterControlFlow {
        (*self).accept_and_identify(visitor, node_id_seq)
    }
}

impl<'ast, T: Visitable<'ast>> Visitable<'ast> for Vec<T>
where
    Node<'ast, Vec<T>>: Into<SqlNode<'ast>>,
{
    fn accept_and_identify(
        &'ast self,
        visitor: &mut dyn VisitorDispatch<'ast>,
        node_id_seq: &mut NodeIdSequence,
    ) -> EnterControlFlow {
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

impl<'ast, T: Visitable<'ast>> Visitable<'ast> for Box<T>
where
    Node<'ast, Box<T>>: Into<SqlNode<'ast>>,
{
    fn accept_and_identify(
        &'ast self,
        visitor: &mut dyn VisitorDispatch<'ast>,
        node_id_seq: &mut NodeIdSequence,
    ) -> EnterControlFlow {
        visit(node_id_seq.next_node(self).into(), visitor, |visitor| {
            (**self).accept_and_identify(visitor, node_id_seq)
        })
    }
}

impl<'ast, T: Visitable<'ast>> Visitable<'ast> for Option<T>
where
    Node<'ast, Option<T>>: Into<SqlNode<'ast>>,
{
    fn accept_and_identify(
        &'ast self,
        visitor: &mut dyn VisitorDispatch<'ast>,
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
