mod import_from_cte;
mod import_from_insert;
mod import_from_table_factor;

use std::{fmt::Debug, marker::PhantomData};

use import_from_cte::*;
use import_from_insert::*;
use import_from_table_factor::*;
use sqlparser::ast::{Expr, Query, SetExpr};
use sqltk::Visitor;

use crate::{
    model::Annotate, model::Projection, model::ResolutionError, model::ScopeOps, model::SourceItem,
    SchemaOps,
};

#[derive(Debug, Visitor)]
#[visitor(
    error_ty = ResolutionError,
    children = [
        ImportFromTableFactor,
        ImportFromInsert,
        ImportFromCte,
    ]
)]
pub struct ImportRelations<'ast, State>(PhantomData<&'ast ()>, PhantomData<State>)
where
    State: Debug
        + ScopeOps
        + Annotate<'ast, Expr, SourceItem>
        + Annotate<'ast, SetExpr, Projection>
        + Annotate<'ast, Query, Projection>
        + SchemaOps;

impl<'ast, State> Default for ImportRelations<'ast, State>
where
    State: Debug
        + ScopeOps
        + Annotate<'ast, Expr, SourceItem>
        + Annotate<'ast, SetExpr, Projection>
        + Annotate<'ast, Query, Projection>
        + SchemaOps,
{
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}
