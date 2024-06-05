use std::rc::Rc;

use crate::ColumnWithOptionalAlias;

/// Records the provenance of a single `SelectItem` node.
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum SelectItemSource {
    ColumnWithOptionalAlias(Rc<ColumnWithOptionalAlias>),

    ResolvedWildcard(Vec<Rc<ColumnWithOptionalAlias>>),
}
