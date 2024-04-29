//! Module that implements the state that is tracked during provenance analysis.

use core::{fmt::Debug, ops::Deref};

use sqlparser::ast::{Ident, SetExpr, Statement};
use sqltk::{
    prelude::{Node, Select},
    Visitable,
};
use std::rc::Rc;

use crate::{
    annotate_sources::SourceAnnotationOps,
    lexical_scope::LexicalScopeOps,
    model::{
        annotations::{AnnotationKey, AnnotationStore},
        resolution_error::ResolutionError,
        schema::Schema,
        sources::{Relation, Source},
    },
    node_path::{NodePath, NodePathEntry, NodePathOps},
    resolution_error::InvariantFailedError,
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
    /// Managages storage and retrieval of `Source` annotations on the AST.
    pub source_annotations: AnnotationStore<'ast, Rc<Source>>,
}

/// Represents the provenance of the projection ([`sqlparser::ast::SelectItem`]s) of a SQL `SELECT`.
///
/// TODO: extend this to cover `INSERT`, `UPDATE` and `DELETE`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Provenance(Vec<Source>);

impl Deref for Provenance {
    type Target = Vec<Source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'ast> ProvenanceState<'ast> {
    /// Get the computed [`Provenance`] of a SQL statement.
    ///
    /// Returns `Ok(provenance)` if successful or `Err(ResolutionError)` if provenance cannot be determined.
    ///
    /// TODO: fix the unimplemented!() calls (by implementing them!)
    pub fn statement_provenance(
        &self,
        statement: &'ast Statement,
    ) -> Result<Provenance, ResolutionError> {
        match statement {
            Statement::Query(query) => match query.body.deref() {
                SetExpr::Select(select) => {
                    let Select { projection, .. } = select.deref();
                    let sources: Result<Vec<Source>, _> = projection
                        .iter()
                        .map(|item| self.get_source(item).cloned())
                        .collect();
                    sources.map(Provenance)
                }
                _ => unimplemented!("SetExpr variant not yet supported"),
            },
            _ => unimplemented!("Statement variant not yet supported"),
        }
    }
}

impl<'ast> From<ProvenanceState<'ast>> for AnnotationStore<'ast, Rc<Source>> {
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

    fn add_relation(&mut self, relation: Relation) -> Result<Relation, ResolutionError> {
        self.scope.add_relation(relation)
    }

    fn resolve_ident(&self, ident: &'ast Ident) -> Result<Source, ResolutionError> {
        self.scope.resolve_ident(ident)
    }

    fn resolve_compound_ident(&self, ident: &'ast [Ident]) -> Result<Source, ResolutionError> {
        self.scope.resolve_compound_ident(ident)
    }

    fn resolve_wildcard(&self) -> Result<Vec<Source>, ResolutionError> {
        self.scope.resolve_wildcard()
    }

    fn resolve_qualified_wildcard(
        &self,
        idents: &'ast [Ident],
    ) -> Result<Vec<Source>, ResolutionError> {
        self.scope.resolve_qualified_wildcard(idents)
    }
}

impl<'ast> SourceAnnotationOps<'ast> for ProvenanceState<'ast>
where
    Self: NodePathOps<'ast>,
{
    fn add_source<N: Visitable<'ast> + Debug>(&mut self, node: &'ast N, source: Source)
    where
        &'ast N: Into<Node<'ast>>,
    {
        let key = AnnotationKey::from(node);

        self.source_annotations
            .entry(key)
            .and_modify(|existing| *existing = Source::merge(&*existing, &source).into())
            .or_insert(Rc::new(source));
    }

    fn get_source<N: Visitable<'ast> + Debug>(
        &self,
        node: &'ast N,
    ) -> Result<&Source, ResolutionError>
    where
        &'ast N: Into<Node<'ast>>,
    {
        let key = AnnotationKey::from(node);
        let sources = self.source_annotations.get(&key);

        match sources {
            Some(sources) => Ok(sources),
            None => Err(ResolutionError::InvariantFailed(
                InvariantFailedError::CouldNotResolveSource(self.get_node_path().to_string()),
            )),
        }
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
