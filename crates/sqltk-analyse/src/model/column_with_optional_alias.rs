use std::rc::Rc;

use crate::{ExprSource, SqlIdent};

use derive_more::AsRef;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, AsRef)]
pub struct ColumnWithOptionalAlias {
    #[as_ref(forward)]
    pub source: Rc<ExprSource>,
    pub alias: Option<Rc<SqlIdent>>,
}

impl ColumnWithOptionalAlias {
    pub fn new(source: Rc<ExprSource>, alias: Option<Rc<SqlIdent>>) -> Self {
        Self { source, alias }
    }
}