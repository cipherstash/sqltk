use crate::*;

impl<'ast, T: Visitable<'ast>> Visitable<'ast> for &'ast T {
    fn accept<VD>(
        &'ast self,
        visitor: &mut VD
    ) -> EnterControlFlow
    where
        VD: VisitorDispatch<'ast>
    {
        (*self).accept(visitor)
    }
}

impl<'ast, T> Visitable<'ast> for Vec<T>
where
    T: Visitable<'ast>,
    &'ast Self: 'ast + Into<SqlNode<'ast>>,
    VecOf<'ast>: From<&'ast Vec<T>>,
{
    fn accept<VD>(
        &'ast self,
        visitor: &mut VD
    ) -> EnterControlFlow
    where
        VD: VisitorDispatch<'ast>
    {
        if self.is_empty() {
            ControlFlow::Continue(Nav::Skip)
        } else {
            visit(SqlNode::from(self), visitor, |visitor| {
                for child in self.iter() {
                    child.accept(visitor)?;
                }
                ControlFlow::Continue(Nav::Visit)
            })
        }
    }
}

impl<'ast, T> Visitable<'ast> for Box<T>
where
    T: Visitable<'ast>,
    &'ast Self: 'ast + Into<SqlNode<'ast>>,
    BoxOf<'ast>: From<&'ast Box<T>>,
{
    fn accept<VD>(
        &'ast self,
        visitor: &mut VD
    ) -> EnterControlFlow
    where
        VD: VisitorDispatch<'ast>
    {
        visit(SqlNode::from(self), visitor, |visitor| {
            (**self).accept(visitor)
        })
    }
}

impl<'ast, T> Visitable<'ast> for Option<T>
where
    T: Visitable<'ast>,
    &'ast Self: 'ast + Into<SqlNode<'ast>>,
    OptionOf<'ast>: From<&'ast Option<T>>,
{
    fn accept<VD>(
        &'ast self,
        visitor: &mut VD
    ) -> EnterControlFlow
    where
        VD: VisitorDispatch<'ast>
    {
        if self.is_none() {
            ControlFlow::Continue(Nav::Skip)
        } else {
            visit(SqlNode::from(self), visitor, |visitor| match self {
                Some(child) => child.accept(visitor),
                None => ControlFlow::Continue(Nav::Skip),
            })
        }
    }
}
