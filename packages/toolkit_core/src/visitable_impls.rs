use crate::*;

impl<'ast, T: Visitable<'ast>> Visitable<'ast> for &'ast T {
    fn accept(
        &'ast self,
        visitor: &mut dyn VisitorDispatch<'ast>,
    ) -> EnterControlFlow {
        (*self).accept(visitor)
    }
}

impl<'ast, T> Visitable<'ast> for Vec<T>
where
    T: Visitable<'ast>,
    &'ast Self: Into<SqlNode<'ast>>,
    VecOf<'ast>: From<&'ast Vec<T>>,
{
    fn accept(
        &'ast self,
        visitor: &mut dyn VisitorDispatch<'ast>,
    ) -> EnterControlFlow {
        if self.is_empty() {
            ControlFlow::Continue(Navigation::Skip)
        } else {
            visit(SqlNode::from(self), visitor, |visitor| {
                for child in self.iter() {
                    child.accept(visitor)?;
                }
                ControlFlow::Continue(Navigation::Visit)
            })
        }
    }
}

impl<'ast, T> Visitable<'ast> for Box<T>
where
    T: Visitable<'ast>,
    &'ast Self: Into<SqlNode<'ast>>,
    BoxOf<'ast>: From<&'ast Box<T>>,
{
    fn accept(
        &'ast self,
        visitor: &mut dyn VisitorDispatch<'ast>,
    ) -> EnterControlFlow {
        visit(SqlNode::from(self), visitor, |visitor| {
            (**self).accept(visitor)
        })
    }
}

impl<'ast, T> Visitable<'ast> for Option<T>
where
    T: Visitable<'ast>,
    &'ast Self: Into<SqlNode<'ast>>,
    OptionOf<'ast>: From<&'ast Option<T>>,
{
    fn accept(
        &'ast self,
        visitor: &mut dyn VisitorDispatch<'ast>,
    ) -> EnterControlFlow {
        if self.is_none() {
            ControlFlow::Continue(Navigation::Skip)
        } else {
            visit(
                SqlNode::from(self),
                visitor,
                |visitor| match self {
                    Some(child) => child.accept(visitor),
                    None => ControlFlow::Continue(Navigation::Skip),
                },
            )
        }
    }
}
