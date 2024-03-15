use crate::*;

impl<'ast, T: Visitable<'ast>> Visitable<'ast> for &'ast T {
    fn accept<State, VD>(
        &'ast self,
        visitor: &VD,
        state: State,
    ) -> VisitorControlFlow<State>
    where
        VD: VisitorDispatch<'ast, State>,
    {
        (*self).accept(visitor, state)
    }
}

impl<'ast, T> Visitable<'ast> for Vec<T>
where
    T: Visitable<'ast>,
    &'ast Self: 'ast + Into<SqlNode<'ast>>,
    VecOf<'ast>: From<&'ast Vec<T>>,
{
    fn accept<State, VD>(
        &'ast self,
        visitor: &VD,
        state: State,
    ) -> VisitorControlFlow<State>
    where
        VD: VisitorDispatch<'ast, State>,
    {
        if self.is_empty() {
            Flow::cont(state)
        } else {
            visit(SqlNode::from(self), visitor, state, |visitor, state| {
                let mut state = state;
                for child in self.iter() {
                    state = child.accept(visitor, state)?;
                }
                Flow::cont(state)
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
    fn accept<State, VD>(
        &'ast self,
        visitor: &VD,
        state: State,
    ) -> VisitorControlFlow<State>
    where
        VD: VisitorDispatch<'ast, State>,
    {
        visit(SqlNode::from(self), visitor, state, |visitor, state| {
            (**self).accept(visitor, state)
        })
    }
}

impl<'ast, T> Visitable<'ast> for Option<T>
where
    T: Visitable<'ast>,
    &'ast Self: 'ast + Into<SqlNode<'ast>>,
    OptionOf<'ast>: From<&'ast Option<T>>,
{
    fn accept<State, VD>(
        &'ast self,
        visitor: &VD,
        state: State,
    ) -> VisitorControlFlow<State>
    where
        VD: VisitorDispatch<'ast, State>,
    {
        if self.is_none() {
            Flow::cont(state)
        } else {
            visit(
                SqlNode::from(self),
                visitor,
                state,
                |visitor, state| match self {
                    Some(child) => child.accept(visitor, state),
                    None => Flow::cont(state),
                },
            )
        }
    }
}
