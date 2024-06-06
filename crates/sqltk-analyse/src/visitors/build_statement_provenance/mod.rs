mod build_delete_provenance;
mod build_insert_provenance;
mod build_select_provenance;
mod build_update_provenance;

use build_delete_provenance::*;
use build_insert_provenance::*;
use build_select_provenance::*;
use build_update_provenance::*;
use sqltk::Visitor;

use std::marker::PhantomData;

use sqlparser::ast::{Expr, Query, SelectItem, Statement};

use crate::{
    model::{Annotate, Projection, ResolutionError, ScopeOps},
    AnnotateMut, ExprSource, Provenance, SchemaOps, SelectItemSource,
};

#[derive(Debug, Visitor)]
#[visitor(
    error_ty = ResolutionError,
    children = [
        BuildInsertProvenance,
        BuildSelectProvenance,
        BuildUpdateProvenance,
        BuildDeleteProvenance,
    ]
)]
pub struct BuildStatementProvenance<'ast, State>(PhantomData<&'ast ()>, PhantomData<State>)
where
    State: ScopeOps
        + Annotate<'ast, Expr, ExprSource>
        + Annotate<'ast, Vec<SelectItem>, Projection>
        + Annotate<'ast, SelectItem, SelectItemSource>
        + Annotate<'ast, Query, Projection>
        + AnnotateMut<'ast, Statement, Provenance>
        + SchemaOps;

impl<'ast, State> Default for BuildStatementProvenance<'ast, State>
where
    State: ScopeOps
        + Annotate<'ast, Expr, ExprSource>
        + Annotate<'ast, Vec<SelectItem>, Projection>
        + Annotate<'ast, SelectItem, SelectItemSource>
        + Annotate<'ast, Query, Projection>
        + AnnotateMut<'ast, Statement, Provenance>
        + SchemaOps,
{
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}