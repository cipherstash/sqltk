include!(concat!(env!("OUT_DIR"), "/generated_visitable_impls.rs"));

use crate::*;

impl<'ast, T> Visitable<'ast> for Vec<T>
where
    T: Visitable<'ast>,
{
    fn accept<V>(
        &'ast self,
        visitor: &V,
        state: V::State,
    ) -> VisitorControlFlow<'ast, V::State, V::Error>
    where
        V: Visitor<'ast>,
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
    N: Visitable<'ast>,
{
    fn accept<V>(
        &'ast self,
        visitor: &V,
        state: V::State,
    ) -> VisitorControlFlow<'ast, V::State, V::Error>
    where
        V: Visitor<'ast>,
    {
        (**self).accept(visitor, state)
    }
}

impl<'ast, N> Visitable<'ast> for Option<N>
where
    N: Visitable<'ast>,
{
    fn accept<V>(
        &'ast self,
        visitor: &V,
        state: V::State,
    ) -> VisitorControlFlow<'ast, V::State, V::Error>
    where
        V: Visitor<'ast>,
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
fn visit<'v, 'ast, N, F, V>(
    node: &'ast N,
    visitor: &'v V,
    state: V::State,
    visit_children: F,
) -> VisitorControlFlow<'ast, V::State, V::Error>
where
    V: Visitor<'ast>,
    N: Visitable<'ast>,
    F: Fn(&V, V::State) -> VisitorControlFlow<'ast, V::State, V::Error>,
{
    let flow = Visitor::enter(visitor, node, state);
    let flow = flow::map_continue(flow, |state| visit_children(visitor, state));

    flow::map_continue(flow, |state| Visitor::exit(visitor, node, state))
}
