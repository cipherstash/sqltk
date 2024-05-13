//! Module that implements the state that is tracked during provenance analysis.

use core::fmt::Debug;

use sqlparser::ast::{Expr, Query, SelectItem, SetExpr, Statement};
use sqltk::prelude::{Node, Select};
use std::{rc::Rc, sync::Arc};

use crate::{
    model::ScopeStack,
    model::SqlIdent,
    model::{Annotates, ExpectedAnnotationError},
    model::{AnnotationStore, NamedRelation, ResolutionError, Schema, ScopeOps, Source},
    model::{Projection, ProjectionColumn},
    node_path::{NodePath, NodePathEntry, NodePathOps},
    provenance::Provenance,
    schema_api::SchemaOps,
};

/// State that is tracked during AST traversal for provenance analysis.
#[derive(Debug)]
pub struct ProvenanceState<'ast> {
    // Keep the internals on the heap to avoid stack overflows.
    inner: Box<InnerState<'ast>>,
}

#[derive(Debug)]
pub struct InnerState<'ast> {
    /// The known database schema.
    schema: Arc<Schema>,
    /// The path through the AST for the current node being visited.
    node_path: NodePath<'ast>,
    /// The current lexical scope.
    scope: ScopeStack,

    expr_sources: AnnotationStore<'ast, Expr, Source>,
    select_item_projection_columns: AnnotationStore<'ast, SelectItem, Vec<Rc<ProjectionColumn>>>,
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
            node_path: Default::default(),
            scope: Default::default(),
            expr_sources: Default::default(),
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

    pub fn of(&self, statement: &'ast Statement) -> Option<Rc<Provenance>> {
        self.inner
            .statement_provenances
            .get_annotation(statement)
            .clone()
    }
}

macro_rules! annotates {
    ($store:ident, $node:ty, $annotation:ty) => {
        impl<'ast> Annotates<'ast, $node, $annotation> for ProvenanceState<'ast>
        where
            AnnotationStore<'ast, $node, $annotation>: Annotates<'ast, $node, $annotation>,
        {
            fn add_annotation(
                &mut self,
                node: &'ast $node,
                annotation: impl Into<Rc<$annotation>>,
            ) -> Rc<$annotation> {
                self.inner.$store.add_annotation(node, annotation.into())
            }

            fn get_annotation(&self, node: &'ast $node) -> Option<Rc<$annotation>> {
                self.inner.$store.get_annotation(node)
            }

            fn expect_annotation(
                &self,
                node: &'ast $node,
            ) -> Result<Rc<$annotation>, ExpectedAnnotationError<$annotation>> {
                self.inner.$store.expect_annotation(node)
            }
        }
    };
}

annotates!(expr_sources, Expr, Source);
annotates!(
    select_item_projection_columns,
    SelectItem,
    Vec<Rc<ProjectionColumn>>
);
annotates!(expr_projections, Expr, Projection);
annotates!(query_projections, Query, Projection);
annotates!(select_projections, Select, Projection);
annotates!(set_expr_projections, SetExpr, Projection);
annotates!(statement_provenances, Statement, Provenance);

impl<'ast> SchemaOps for ProvenanceState<'ast> {
    fn get_schema(&self) -> &Schema {
        &self.inner.schema
    }
}

impl<'ast> ScopeOps<'ast> for ProvenanceState<'ast> {
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

    fn resolve_ident(&self, ident: &SqlIdent) -> Result<Rc<Source>, ResolutionError> {
        self.inner.scope.resolve_ident(ident)
    }

    fn resolve_compound_ident(&self, ident: &[SqlIdent]) -> Result<Rc<Source>, ResolutionError> {
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

    #[cfg(test)]
    fn dump_scope(&self) {
        self.inner.scope.dump()
    }
}

impl<'ast> NodePathOps<'ast> for ProvenanceState<'ast> {
    fn push_path_entry<N>(&mut self, node: &'ast N)
    where
        &'ast N: Into<Node<'ast>>,
    {
        self.inner.node_path.push_path_entry(node)
    }

    fn pop_path_entry(&mut self) -> Option<NodePathEntry<'ast>> {
        self.inner.node_path.pop_path_entry()
    }

    fn peek_path_entry(&self) -> Option<&NodePathEntry<'ast>> {
        self.inner.node_path.peek_path_entry()
    }

    fn get_node_path(&self) -> &NodePath<'ast> {
        &self.inner.node_path
    }
}
