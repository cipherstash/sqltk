mod import_from_cte;
mod import_from_insert;
mod import_from_table_factor;

use std::fmt::Debug;

use import_from_cte::*;
use import_from_insert::*;
use import_from_table_factor::*;
use sqlparser::ast::{Expr, Query, SetExpr};
use sqltk::{generalise, VisitorStack};

use derive_more::Deref;

use crate::{
    model::Annotates, model::Projection, model::ResolutionError, model::ScopeOps, model::Source,
    SchemaOps,
};

#[derive(Default, Debug, Deref)]
pub struct ImportIdentifiers<'ast, State> {
    stack: VisitorStack<'ast, State, ResolutionError>,
}

impl<'ast, State: Debug> ImportIdentifiers<'ast, State>
where
    State: 'ast
        + Debug
        + ScopeOps<'ast>
        + Annotates<'ast, Expr, Source>
        + Annotates<'ast, SetExpr, Projection>
        + Annotates<'ast, Query, Projection>
        + SchemaOps,
{
    pub fn new() -> Self {
        let mut stack = VisitorStack::new();
        stack.push(generalise(ImportFromTableFactor));
        stack.push(generalise(ImportFromInsert));
        stack.push(generalise(ImportFromCte));
        Self { stack }
    }
}
