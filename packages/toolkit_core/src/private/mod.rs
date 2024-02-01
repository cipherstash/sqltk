use std::ops::ControlFlow;

use crate::{SqlNode, EnterControlFlow, Navigation, VisitorDispatch};

#[inline(always)]
pub fn visit<'ast, F>(
    node: SqlNode<'ast>,
    visitor: &mut dyn VisitorDispatch<'ast>,
    mut visit_children: F,
) -> EnterControlFlow
where
    F: FnMut(&mut dyn VisitorDispatch<'ast>) -> EnterControlFlow,
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
