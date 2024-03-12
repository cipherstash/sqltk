use crate::{Enter, EnterControlFlow, ExitControlFlow, Nav, SqlNode, VisitorDispatch};

#[inline(always)]
pub fn visit<'ast, F, VD>(node: SqlNode<'ast>, visitor: &mut VD, visit_children: F) -> EnterControlFlow
where
    VD: VisitorDispatch<'ast>,
    F: Fn(&mut VD) -> EnterControlFlow,
{
    let child_nodes_control_flow = match VisitorDispatch::enter(visitor, node.clone()) {
        EnterControlFlow::Continue(Nav::Visit) => visit_children(visitor),
        other => other,
    };

    if child_nodes_control_flow.is_break() {
        child_nodes_control_flow
    } else {
        match VisitorDispatch::exit(visitor, node) {
            ExitControlFlow::Continue(()) => child_nodes_control_flow,
            ExitControlFlow::Break(err) => Enter::error(err),
        }
    }
}
