use std::ops::ControlFlow;

use crate::{ConcreteNode, EnterControlFlow, Navigation, VisitorDispatch};

pub fn visit<'ast, V, F>(
    node: ConcreteNode<'ast>,
    visitor: &mut V,
    mut visit_children: F,
) -> EnterControlFlow
where
    V: VisitorDispatch<'ast>,
    F: FnMut(&mut V) -> EnterControlFlow,
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
