mod build_query_projection;
mod build_select_projection;
mod build_set_expr_projection;
mod build_vec_of_select_item_projection;

use build_query_projection::*;
use build_select_projection::*;
use build_set_expr_projection::*;
use build_vec_of_select_item_projection::*;

use sqlparser::ast::{Expr, Query, Select, SelectItem, SetExpr};

use sqltk::Visitor;

use crate::{
    model::{Annotate, Projection, ScopeOps, Source},
    AnnotateMut, ProjectionColumn, ResolutionError, SchemaOps,
};

use std::{fmt::Debug, marker::PhantomData, rc::Rc};

#[derive(Debug, Visitor)]
#[visitor(
    error_ty = ResolutionError,
    children = [
        BuildVecOfSelectItemProjection,
        BuildSetExprProjection,
        BuildSelectProjection,
        BuildQueryProjection,
    ]
)]
pub struct BuildProjections<'ast, State>(PhantomData<&'ast ()>, PhantomData<State>)
where
    State: Debug
        + ScopeOps
        + Annotate<'ast, Expr, Source>
        + Annotate<'ast, SelectItem, Vec<Rc<ProjectionColumn>>>
        + AnnotateMut<'ast, Query, Projection>
        + AnnotateMut<'ast, Select, Projection>
        + AnnotateMut<'ast, Vec<SelectItem>, Projection>
        + AnnotateMut<'ast, SetExpr, Projection>
        + SchemaOps;

impl<'ast, State> Default for BuildProjections<'ast, State>
where
    State: Debug
        + ScopeOps
        + Annotate<'ast, Expr, Source>
        + Annotate<'ast, SelectItem, Vec<Rc<ProjectionColumn>>>
        + AnnotateMut<'ast, Query, Projection>
        + AnnotateMut<'ast, Select, Projection>
        + AnnotateMut<'ast, Vec<SelectItem>, Projection>
        + AnnotateMut<'ast, SetExpr, Projection>
        + SchemaOps,
{
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}
