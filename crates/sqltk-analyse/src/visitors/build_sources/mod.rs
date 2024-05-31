mod build_expr_sources;
mod build_select_item_sources;

use build_expr_sources::*;
use build_select_item_sources::*;

use sqlparser::ast::{Expr, Query, SelectItem};

use sqltk::Visitor;

use crate::{
    model::{Annotate, ExprSource, Projection, ScopeOps},
    AnnotateMut, ResolutionError, SchemaOps, SelectItemSource,
};

use std::{fmt::Debug, marker::PhantomData};

#[derive(Debug, Visitor)]
#[visitor(
    error_ty = ResolutionError,
    children = [
        BuildExprSources,
        BuildSelectItemSources
    ]
)]
pub struct BuildSources<'ast, State>(PhantomData<&'ast ()>, PhantomData<State>)
where
    State: Debug
        + ScopeOps
        + AnnotateMut<'ast, Expr, ExprSource>
        + AnnotateMut<'ast, SelectItem, SelectItemSource>
        + Annotate<'ast, Query, Projection>
        + SchemaOps;

impl<'ast, State> Default for BuildSources<'ast, State>
where
    State: Debug
        + ScopeOps
        + AnnotateMut<'ast, Expr, ExprSource>
        + AnnotateMut<'ast, SelectItem, SelectItemSource>
        + Annotate<'ast, Query, Projection>
        + SchemaOps,
{
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}
