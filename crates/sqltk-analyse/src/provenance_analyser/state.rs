//! Module that implements the state that is tracked during provenance analysis.

use core::fmt::Debug;

use sqlparser::ast::{Expr, Query, SelectItem, SetExpr, Statement};
use sqltk::prelude::Select;
use std::{rc::Rc, sync::Arc};

use crate::model::{
    Annotate, AnnotateMut, AnnotationStore, ExpectedAnnotationError, NamedRelation, Projection,
    ProjectionColumn, Provenance, ResolutionError, Schema, SchemaOps, ScopeOps, ScopeStack,
    SourceItem, SqlIdent,
};

/// State that is tracked during AST traversal for provenance analysis.
#[derive(Debug)]
pub struct ProvenanceState<'ast> {
    // Keep the internals on the heap to avoid stack overflows.
    inner: Box<InnerState<'ast>>,
}

pub trait ProvenanceStateBounds<'ast>
where
    Self: ScopeOps
        + AnnotateMut<'ast, Expr, SourceItem>
        + AnnotateMut<'ast, SelectItem, Vec<Rc<ProjectionColumn>>>
        + AnnotateMut<'ast, Vec<SelectItem>, Projection>
        + AnnotateMut<'ast, Expr, Projection>
        + AnnotateMut<'ast, Query, Projection>
        + AnnotateMut<'ast, SetExpr, Projection>
        + AnnotateMut<'ast, Select, Projection>
        + AnnotateMut<'ast, Statement, Provenance>
        + SchemaOps
        + Debug,
{
}

impl<'ast> ProvenanceStateBounds<'ast> for ProvenanceState<'ast> {}

#[derive(Debug)]
pub struct InnerState<'ast> {
    /// The known database schema.
    schema: Arc<Schema>,
    /// The current lexical scope.
    scope: ScopeStack,

    expr_sources: AnnotationStore<'ast, Expr, SourceItem>,
    select_item_projection_columns: AnnotationStore<'ast, SelectItem, Vec<Rc<ProjectionColumn>>>,
    vec_of_select_item_projections: AnnotationStore<'ast, Vec<SelectItem>, Projection>,
    expr_projections: AnnotationStore<'ast, Expr, Projection>,
    query_projections: AnnotationStore<'ast, Query, Projection>,
    select_projections: AnnotationStore<'ast, Select, Projection>,
    set_expr_projections: AnnotationStore<'ast, SetExpr, Projection>,
    statement_provenances: AnnotationStore<'ast, Statement, Provenance>,
}

impl<'ast> InnerState<'ast> {
    fn new(schema: impl Into<Arc<Schema>>) -> Self {
        Self {
            schema: schema.into(),
            scope: Default::default(),
            expr_sources: Default::default(),
            vec_of_select_item_projections: Default::default(),
            select_item_projection_columns: Default::default(),
            expr_projections: Default::default(),
            query_projections: Default::default(),
            select_projections: Default::default(),
            set_expr_projections: Default::default(),
            statement_provenances: Default::default(),
        }
    }
}

impl<'ast> ProvenanceState<'ast> {
    pub fn new(schema: impl Into<Arc<Schema>>) -> Self {
        Self {
            inner: InnerState::new(schema).into(),
        }
    }
}

macro_rules! annotate {
    ($store:ident, $node:ty, $annotation:ty) => {
        impl<'ast> Annotate<'ast, $node, $annotation> for ProvenanceState<'ast>
        where
            AnnotationStore<'ast, $node, $annotation>: Annotate<'ast, $node, $annotation>,
        {
            fn get_annotation(
                &self,
                node: &'ast $node,
            ) -> Result<Rc<$annotation>, ExpectedAnnotationError<$annotation>> {
                self.inner.$store.get_annotation(node)
            }
        }

        impl<'ast> AnnotateMut<'ast, $node, $annotation> for ProvenanceState<'ast>
        where
            AnnotationStore<'ast, $node, $annotation>: Annotate<'ast, $node, $annotation>,
        {
            fn set_annotation(
                &mut self,
                node: &'ast $node,
                annotation: impl Into<Rc<$annotation>>,
            ) -> Rc<$annotation> {
                self.inner.$store.set_annotation(node, annotation.into())
            }
        }
    };
}

annotate!(expr_sources, Expr, SourceItem);
annotate!(
    select_item_projection_columns,
    SelectItem,
    Vec<Rc<ProjectionColumn>>
);
annotate!(vec_of_select_item_projections, Vec<SelectItem>, Projection);
annotate!(expr_projections, Expr, Projection);
annotate!(query_projections, Query, Projection);
annotate!(select_projections, Select, Projection);
annotate!(set_expr_projections, SetExpr, Projection);
annotate!(statement_provenances, Statement, Provenance);

impl<'ast> SchemaOps for ProvenanceState<'ast> {
    fn get_schema(&self) -> &Schema {
        &self.inner.schema
    }
}

impl<'ast> ScopeOps for ProvenanceState<'ast> {
    fn reset_scope(&mut self) {
        self.inner.scope.reset();
    }

    fn push_scope(&mut self) {
        self.inner.scope.push();
    }

    fn pop_scope(&mut self) {
        self.inner.scope.pop();
    }

    fn add_relation(
        &mut self,
        relation: Rc<NamedRelation>,
    ) -> Result<Rc<NamedRelation>, ResolutionError> {
        self.inner.scope.add_relation(relation)
    }

    fn resolve_relation(&mut self, name: &SqlIdent) -> Result<Rc<NamedRelation>, ResolutionError> {
        self.inner.scope.resolve_relation(name)
    }

    fn resolve_ident(&self, ident: &SqlIdent) -> Result<Rc<SourceItem>, ResolutionError> {
        self.inner.scope.resolve_ident(ident)
    }

    fn resolve_compound_ident(
        &self,
        ident: &[SqlIdent],
    ) -> Result<Rc<SourceItem>, ResolutionError> {
        self.inner.scope.resolve_compound_ident(ident)
    }

    fn resolve_wildcard(&self) -> Result<Rc<Projection>, ResolutionError> {
        self.inner.scope.resolve_wildcard()
    }

    fn resolve_qualified_wildcard(
        &self,
        idents: &[SqlIdent],
    ) -> Result<Rc<Projection>, ResolutionError> {
        self.inner.scope.resolve_qualified_wildcard(idents)
    }
}
