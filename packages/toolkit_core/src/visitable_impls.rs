use crate::*;

// impl<'ast, T: Visitable> Visitable for &'ast T {
//     fn accept<'ast, 'dispatch>(
//         &'ast self,
//         visitor: &'dispatch mut dyn VisitorDispatch,
//     ) -> EnterControlFlow
//     where
//         'ast: 'dispatch
//     {
//         (*self).accept(visitor)
//     }
// }

impl<'a, T: 'a> Visitable for Vec<T>
where
    T: Visitable,
    // &'a Self: Into<SqlNode<'a>>,
    for<'x> VecOf<'x>: From<&'x Vec<T>>,
{
    fn accept<'ast, 'dispatch>(
        &'ast self,
        visitor: &'dispatch mut dyn VisitorDispatch,
    ) -> EnterControlFlow
    where
        'ast: 'dispatch,
    {
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

impl<'a, T: 'a> Visitable for Box<T>
where
    T: Visitable,
    for<'any> &'any Self: Into<SqlNode<'any>>,
    for<'any> BoxOf<'any>: From<&'any Box<T>>,
{
    fn accept<'ast, 'dispatch>(
        &'ast self,
        visitor: &'dispatch mut dyn VisitorDispatch,
    ) -> EnterControlFlow
    where
        'ast: 'dispatch,
    {
        visit(SqlNode::from(self), visitor, |visitor| {
            (**self).accept(visitor)
        })
    }
}

impl<'a, T: 'a> Visitable for Option<T>
where
    T: Visitable,
    for<'any> &'any Self: Into<SqlNode<'any>>,
    for<'any> OptionOf<'any>: From<&'any Option<T>>,
{
    fn accept<'ast, 'dispatch>(
        &'ast self,
        visitor: &'dispatch mut dyn VisitorDispatch,
    ) -> EnterControlFlow
    where
        'ast: 'dispatch,
    {
        if self.is_none() {
            ControlFlow::Continue(Navigation::Skip)
        } else {
            visit(SqlNode::from(self), visitor, |visitor| match self {
                Some(child) => child.accept(visitor),
                None => ControlFlow::Continue(Navigation::Skip),
            })
        }
    }
}
