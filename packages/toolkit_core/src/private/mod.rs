use crate::{Flow, SqlNode, VisitorControlFlow, VisitorDispatch};

#[inline(always)]
pub fn visit<'ast, F, VD, State>(
    node: SqlNode<'ast>,
    visitor: &VD,
    state: State,
    visit_children: F,
) -> VisitorControlFlow<State>
where
    VD: VisitorDispatch<'ast, State>,
    F: Fn(&VD, State) -> VisitorControlFlow<State>,
{
    let flow = VisitorDispatch::enter(visitor, node.clone(), state);
    let flow = Flow::map_continue(flow, |state| visit_children(visitor, state));
    let flow = Flow::map_continue(flow, |state| {
        VisitorDispatch::exit(visitor, node.clone(), state)
    });
    flow
}
