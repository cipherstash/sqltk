include!(concat!(env!("OUT_DIR"), "/generated_visitable_impls.rs"));

use crate::*;

impl<'ast, T> Visitable<'ast> for Vec<T>
where
    T: 'static + Visitable<'ast>,
    VecOf<'ast>: From<&'ast Vec<T>>,
{
    fn accept<State, E, V>(
        &'ast self,
        visitor: &V,
        state: State,
    ) -> VisitorControlFlow<'ast, State, E>
    where
        E: Error + Debug,
        V: Visitor<'ast, State, E>,
    {
        if self.is_empty() {
            flow::cont(state)
        } else {
            visit(self, visitor, state, |visitor, state| {
                let mut state = state;
                for child in self.iter() {
                    state = child.accept(visitor, state)?;
                }
                flow::cont(state)
            })
        }
    }
}

impl<'ast, N> Visitable<'ast> for Box<N>
where
    &'ast N: Into<Node<'ast>>,
    N: 'static + Visitable<'ast>,
{
    fn accept<State, E, V>(
        &'ast self,
        visitor: &V,
        state: State,
    ) -> VisitorControlFlow<'ast, State, E>
    where
        E: Error + Debug,
        V: Visitor<'ast, State, E>,
    {
        (**self).accept(visitor, state)
    }
}

impl<'ast, N> Visitable<'ast> for Option<N>
where
    N: 'static + Visitable<'ast>,
    &'ast Option<N>: Into<Node<'ast>>,
{
    fn accept<State, E, V>(
        &'ast self,
        visitor: &V,
        state: State,
    ) -> VisitorControlFlow<'ast, State, E>
    where
        E: Error + Debug,
        V: Visitor<'ast, State, E>,
    {
        if self.is_none() {
            flow::cont(state)
        } else {
            visit(self, visitor, state, |visitor, state| match self {
                Some(child) => child.accept(visitor, state),
                None => flow::cont(state),
            })
        }
    }
}

/// Helper function used by generated code for visiting a node and its children
/// recursively while properly handling visitor control flow.
/// FIXME: put this code in the Visitable trait - it will simplify the generics.
#[inline(always)]
fn visit<'v, 'ast, N, F, E, V, State>(
    node: &'ast N,
    visitor: &'v V,
    state: State,
    visit_children: F,
) -> VisitorControlFlow<'ast, State, E>
where
    E: Error + Debug,
    N: 'static + Visitable<'ast>,
    &'ast N: Into<Node<'ast>>,
    V: Visitor<'ast, State, E>,
    F: Fn(&V, State) -> VisitorControlFlow<'ast, State, E>,
{
    let flow = Visitor::enter(visitor, node, state);
    let flow = flow::map_continue(flow, |state| visit_children(visitor, state));

    flow::map_continue(flow, |state| Visitor::exit(visitor, node, state))
}
