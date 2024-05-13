use std::rc::Rc;

use crate::{
    model::projection_annotation::Projection,
    model::resolution_error::ResolutionError,
    model::schema::SqlIdent,
    model::source_annotation::{NamedRelation, Source},
};

/// Operations for manipulating lexical scope and resolving identifiers that are in-scope.
pub trait ScopeOps<'ast> {
    /// Resets the scope stack. This is called between every `Statement`.
    fn reset_scope(&mut self);

    /// Pushes a new empty scope onto the stack.
    fn push_scope(&mut self);

    /// Pops the top scope off the stack.
    fn pop_scope(&mut self);

    /// Puts a table/view/subquery projection into scope.
    fn add_relation(
        &mut self,
        relation: Rc<NamedRelation>,
    ) -> Result<Rc<NamedRelation>, ResolutionError>;

    /// Resolves a relation that is either in-scope (such as an aliased subquery or CTE)
    /// falling back to the database schema.
    fn resolve_relation(&mut self, name: &SqlIdent) -> Result<Rc<NamedRelation>, ResolutionError>;

    /// Resolves an identifier within the current scope.
    fn resolve_ident(&self, ident: &SqlIdent) -> Result<Rc<Source>, ResolutionError>;

    /// Resolves a compound identifier within the current scope.
    fn resolve_compound_ident(&self, ident: &[SqlIdent]) -> Result<Rc<Source>, ResolutionError>;

    /// Expands a wildcard within the current scope.
    fn resolve_wildcard(&self) -> Result<Rc<Projection>, ResolutionError>;

    /// Expands a qualified wildcard within the current scope.
    fn resolve_qualified_wildcard(
        &self,
        ident: &[SqlIdent],
    ) -> Result<Rc<Projection>, ResolutionError>;

    #[cfg(test)]
    fn dump_scope(&self);
}
