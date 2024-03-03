use std::ops::ControlFlow;

use crate::{EnterControlFlow, Navigation, SqlNode, VisitorDispatch};

#[inline(always)]
pub fn visit<'ast, 'dispatch, F>(
    node: SqlNode<'ast>,
    visitor: &'dispatch mut dyn VisitorDispatch,
    visit_children: F,
) -> EnterControlFlow
where
    'ast: 'dispatch,
    F: FnOnce(&mut dyn VisitorDispatch) -> EnterControlFlow,
{
    let child_nodes_control_flow = match VisitorDispatch::enter(visitor, node.clone()) {
        ControlFlow::Continue(Navigation::Visit) => visit_children(visitor),
        other => other,
    };

    if child_nodes_control_flow.is_break() {
        child_nodes_control_flow
    } else {
        match VisitorDispatch::exit(visitor, node) {
            ControlFlow::Continue(()) => child_nodes_control_flow,
            _ => ControlFlow::Break(()),
        }
    }
}
