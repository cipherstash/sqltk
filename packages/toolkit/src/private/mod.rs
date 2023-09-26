use std::ops::ControlFlow;

use crate::{ConcreteNode, Navigation, VisitorControlFlow, VisitorDispatch};

pub fn visit<'ast, V, F>(
    node: ConcreteNode<'ast>,
    visitor: &V,
    visit_children: F,
) -> VisitorControlFlow
where
    V: VisitorDispatch<'ast>,
    F: FnOnce() -> VisitorControlFlow,
{
    let result = match visitor.dispatch_enter(node.clone()) {
        ControlFlow::Continue(Navigation::Visit) => visit_children(),
        other => other,
    };

    if result.is_break() {
        result
    } else {
        visitor.dispatch_exit(node)
    }
}
