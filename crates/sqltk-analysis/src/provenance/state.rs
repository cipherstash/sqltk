//! Module that implements the state that is tracked during provenance analysis.

use core::{fmt::Debug, ops::Deref};

use sqlparser::ast::{Expr, Ident, Query, SelectItem, SetExpr, Statement};
use sqltk::{
    prelude::{Node, Select},
    Visitable,
};
use std::rc::Rc;
use unicase::UniCase;

use crate::{
    annotations::{Annotates, ExpectedAnnotationError},
    lexical_scope::LexicalScopeOps,
    model::{
        annotations::AnnotationStore,
        resolution_error::ResolutionError,
        schema::Schema,
        source_annotation::{NamedRelation, SourceAnnotation},
    },
    node_path::{NodePath, NodePathEntry, NodePathOps},
    projection_annotation::{Projection, ProjectionAnnotation},
    schema_api::SchemaOps,
    scope_stack::ScopeStack,
};

/// State that is tracked during AST traversal for provenance analysis.
#[derive(Default, Debug)]
pub struct ProvenanceState<'ast> {
    /// The known database schema.
    pub schema: Schema,
    /// The path through the AST for the current node being visited.
    pub node_path: NodePath<'ast>,
    /// The current lexical scope.
    pub scope: ScopeStack,
    /// Manages storage and retrieval of `SourceAnnotation` annotations on the AST.
    pub source_annotations: AnnotationStore<'ast, Rc<SourceAnnotation>>,
    /// Manages storage and retrieval of `ProjectionAnnotaton` annotations on the AST.
    pub projection_annotations: AnnotationStore<'ast, Rc<ProjectionAnnotation>>,
}

impl<'ast> ProvenanceState<'ast> {
    /// Get the computed [`Provenance`] of a SQL statement.
    ///
    /// Returns `Ok(provenance)` if successful or `Err(ResolutionError)` if
    /// provenance cannot be determined.
    pub fn statement_provenance(
        &self,
        statement: &'ast Statement,
    ) -> Result<Projection, ResolutionError> {
        match statement {
            Statement::Query(query) => {
                let projection: Result<Rc<ProjectionAnnotation>, _> =
                    self.expect_annotation(query.deref());
                match projection {
                    Ok(annotation) => match annotation.deref() {
                        ProjectionAnnotation::Query(projection) => Ok(projection.clone()),
                        _ => Err(ResolutionError::Unimplemented),
                    },
                    Err(err) => Err(err.into()),
                }
            }
            _ => Err(ResolutionError::Unimplemented),
        }
    }
}

macro_rules! annotates {
    ($store:ident, $node:ty, $annotation:ty) => {
        impl<'ast> Annotates<'ast, $node, $annotation> for ProvenanceState<'ast>
        where
            AnnotationStore<'ast, Rc<$annotation>>: Annotates<'ast, $node, $annotation>,
        {
            fn add_annotation(
                &mut self,
                node: &'ast $node,
                annotation: impl Into<Rc<$annotation>>,
            ) -> Rc<$annotation> {
                self.$store.add_annotation(node, annotation.into())
            }

            fn get_annotation(&self, node: &'ast $node) -> Option<Rc<$annotation>> {
                self.$store.get_annotation(node)
            }

            fn expect_annotation(
                &self,
                node: &'ast $node,
            ) -> Result<Rc<$annotation>, ExpectedAnnotationError<$annotation>> {
                self.$store.expect_annotation(node)
            }
        }
    };
}

annotates!(source_annotations, Expr, SourceAnnotation);
annotates!(source_annotations, SelectItem, SourceAnnotation);
annotates!(projection_annotations, Expr, ProjectionAnnotation);
annotates!(projection_annotations, Query, ProjectionAnnotation);
annotates!(projection_annotations, Select, ProjectionAnnotation);
annotates!(projection_annotations, SetExpr, ProjectionAnnotation);

impl<'ast> From<ProvenanceState<'ast>> for AnnotationStore<'ast, Rc<SourceAnnotation>> {
    fn from(value: ProvenanceState<'ast>) -> Self {
        value.source_annotations
    }
}

impl<'ast> SchemaOps for ProvenanceState<'ast> {
    fn get_schema(&self) -> &Schema {
        &self.schema
    }
}

impl<'ast> LexicalScopeOps<'ast> for ProvenanceState<'ast> {
    fn reset_scope(&mut self) {
        self.scope.reset();
    }

    fn push_scope(&mut self) {
        self.scope.push();
    }

    fn pop_scope(&mut self) {
        self.scope.pop();
    }

    fn add_relation(&mut self, relation: NamedRelation) -> Result<NamedRelation, ResolutionError> {
        self.scope.add_relation(relation)
    }

    fn resolve_relation(
        &mut self,
        name: &UniCase<String>,
    ) -> Result<NamedRelation, ResolutionError> {
        self.scope.resolve_relation(name)
    }

    fn resolve_ident(&self, ident: &'ast Ident) -> Result<Rc<SourceAnnotation>, ResolutionError> {
        self.scope.resolve_ident(ident)
    }

    fn resolve_compound_ident(
        &self,
        ident: &'ast [Ident],
    ) -> Result<Rc<SourceAnnotation>, ResolutionError> {
        self.scope.resolve_compound_ident(ident)
    }

    fn resolve_wildcard(&self) -> Result<Projection, ResolutionError> {
        self.scope.resolve_wildcard()
    }

    fn resolve_qualified_wildcard(
        &self,
        idents: &'ast [Ident],
    ) -> Result<Projection, ResolutionError> {
        self.scope.resolve_qualified_wildcard(idents)
    }
}

impl<'ast> NodePathOps<'ast> for ProvenanceState<'ast> {
    fn push_path_entry<N: Visitable<'ast>>(&mut self, node: &'ast N)
    where
        &'ast N: Into<Node<'ast>>,
    {
        self.node_path.push_path_entry(node)
    }

    fn pop_path_entry(&mut self) -> Option<NodePathEntry<'ast>> {
        self.node_path.pop_path_entry()
    }

    fn peek_path_entry(&self) -> Option<&NodePathEntry<'ast>> {
        self.node_path.peek_path_entry()
    }

    fn get_node_path(&self) -> &NodePath<'ast> {
        &self.node_path
    }
}
