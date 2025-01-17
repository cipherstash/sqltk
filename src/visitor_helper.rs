use std::ops::ControlFlow;

use crate::{Break, Visitable, Visitor};

/// Helper function used by generated code for visiting a node and its children
/// recursively while properly handling visitor control flow.
#[inline(always)]
pub(crate) fn visit<'ast, N, F, V>(
    node: &'ast N,
    visitor: &mut V,
    visit_children: F,
) -> ControlFlow<Break<V::Error>>
where
    V: Visitor<'ast>,
    N: Visitable,
    F: Fn(&mut V) -> ControlFlow<Break<V::Error>>,
{
    visitor.enter(node)?;
    visit_children(visitor)?;
    visitor.exit(node)
}
