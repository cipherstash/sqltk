use std::ops::ControlFlow;

use crate::{ConcreteNode, Navigation, VisitorControlFlow, VisitorDispatch};

pub fn visit<'ast, V, F>(
    node: ConcreteNode<'ast>,
    visitor: &mut V,
    mut visit_children: F,
) -> VisitorControlFlow
where
    V: VisitorDispatch<'ast>,
    F: FnMut(&mut V) -> VisitorControlFlow,
{
    let result = match visitor.dispatch_enter(node.clone()) {
        ControlFlow::Continue(Navigation::Visit) => visit_children(visitor),
        other => other,
    };

    if result.is_break() {
        result
    } else {
        visitor.dispatch_exit(node)
    }
}