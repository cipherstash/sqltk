use std::ops::ControlFlow;

use crate::{EnterControlFlow, Navigation, SqlNode, VisitorDispatch};

#[inline(always)]
pub fn visit<'state, 'ast: 'state, F>(
    node: SqlNode<'ast>,
    visitor: &mut dyn VisitorDispatch<'state, 'ast>,
    visit_children: F,
) -> EnterControlFlow
where
    F: Fn(&mut dyn VisitorDispatch<'state, 'ast>) -> EnterControlFlow,
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
