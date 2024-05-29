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
    ) -> ControlFlow<Break<V::State, V::Error>, V::State>
    where
        V: Visitor<'ast>,
    {
        use crate::visitor_extensions::VisitorExtensions;

        if self.is_empty() {
            visitor.continue_with_state(state)
        } else {
            visit(self, visitor, state, |visitor, state| {
                let mut state = state;
                for child in self.iter() {
                    state = child.accept(visitor, state)?;
                }
                visitor.continue_with_state(state)
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
    ) -> ControlFlow<Break<V::State, V::Error>, V::State>
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
    ) -> ControlFlow<Break<V::State, V::Error>, V::State>
    where
        V: Visitor<'ast>,
    {
        use crate::visitor_extensions::VisitorExtensions;

        if self.is_none() {
            visitor.continue_with_state(state)
        } else {
            visit(self, visitor, state, |visitor, state| match self {
                Some(child) => child.accept(visitor, state),
                None => visitor.continue_with_state(state),
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
) -> ControlFlow<Break<V::State, V::Error>, V::State>
where
    V: Visitor<'ast>,
    N: Visitable<'ast>,
    F: Fn(&V, V::State) -> ControlFlow<Break<V::State, V::Error>, V::State>,
{
    let state = visitor.enter(node, state)?;
    let state = visit_children(visitor, state)?;
    visitor.exit(node, state)
}
