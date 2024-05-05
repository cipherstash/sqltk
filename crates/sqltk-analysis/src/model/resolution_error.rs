//! Errors that can be returned during resolution of identifiers in scope or
//! when attempting to resolve the [`crate::model::sources::Source`] of an
//! [`sqlparser::ast::Expr`].

use std::rc::Rc;

use sqlparser::ast::{Expr, Query, SetExpr};

use crate::{
    model::annotation_store::ExpectedAnnotationError,
    model::projection_annotation::{Projection, ProjectionColumn},
    model::schema::{AmbiguousMatchError, FindUniqueMatchError},
    model::source_annotation::Source,
    model::Provenance,
};

/// Error that can be returned when either:
/// - trying to resolve identifiers in scope, or
/// - trying to add the same table/view/aliased item to scope more than once
/// - some internal invariant failed
///
/// Regarding failed invariants: they 100% represent internal bugs and therefore
/// should not be part of the API (invariant failures should trigger a panic)
/// and those variants will be removed when the `sqltk-analysis` stabilises.
#[derive(Debug, thiserror::Error, Eq, PartialEq, PartialOrd, Ord)]
pub enum ResolutionError {
    #[error("Ambiguous identifier: {}", _0)]
    AmbiguousIdentifier(String),

    #[error("Unknown identifier: {}", _0)]
    NoSuchIdentifier(String),

    #[error("No such relation: {}", _0)]
    NoSuchRelation(String),

    #[error("Unknown compound identifier: {}", _0)]
    NoSuchCompoundIdentifier(String),

    #[error("Tried to add table column that was already in scope: {}", _0)]
    AlreadyInScope(String),

    #[error("Something went wrong: {}", _0)]
    InvariantFailed(#[from] InvariantFailedError),

    #[error("Missing source annotation: {}", _0)]
    ExpectedSourceAnnotation(#[from] ExpectedAnnotationError<Source>),

    #[error("Missing statement annotation: {}", _0)]
    ExpectedProvenanceAnnotation(#[from] ExpectedAnnotationError<Provenance>),

    #[error("Missing projection annotation: {}", _0)]
    ExpectedProjectionAnnotation(#[from] ExpectedAnnotationError<Projection>),

    #[error("Missing projection column annotation: {}", _0)]
    ExpectedProjectionColumnAnnotation(#[from] ExpectedAnnotationError<Vec<Rc<ProjectionColumn>>>),

    #[error("Invalid subquery expression (selects more than one column) {}", _0)]
    InvalidSubqueryExpr(Box<Query>),

    // If we got here the parse should have failed - so have a think about how
    // we can remove this variant.
    #[error("Invalid subquery expression (no columns) {}", _0)]
    InvalidArraySubqueryExpr(Box<Query>),

    #[error(
        "Support for INSERT OR UPDATE in subquery expression position is currently unimplemented"
    )]
    UnsupportedInsertOrUpdateInSubqueryExpressionPosition(Box<SetExpr>),

    #[error("Support for TABLE in subquery expression position is currently unimplemented")]
    UnsupportedTableKeywordInSubqueryExpressionPosition(Box<SetExpr>),

    #[error("Unexpected statement variant in SetExpr; expected: {}", _0)]
    UnpexpectedStatementInSetExpr(String, Box<SetExpr>),

    #[error("Unsupported compound identifier length: {}", _0.iter().map(|id| id.to_string()).collect::<Vec<_>>().join("."))]
    UnsupportedCompoundIdentifierLength(Vec<String>),

    #[error("Unresolvable wildcard: {}", _0)]
    UnresolvableWildcard(Box<Expr>),

    #[error(
        "Incompatible operands for set operation; left: [{}] right: [{}]",
        _0,
        _1
    )]
    IncompatibleOperandsForSetOperation(Box<SetExpr>, Box<SetExpr>),

    #[error("Empty projection in subquery: [{}]", _0)]
    EmptyProjection(Box<Query>),

    #[error("Support for all SQL syntax is not yet complete")]
    Unimplemented,

    #[error("Support for all SQL syntax is not yet complete")]
    UnsupportTableAliasVariant,
}

#[derive(Debug, thiserror::Error, Eq, PartialEq, PartialOrd, Ord)]
pub enum InvariantFailedError {
    #[error("Could not resolve source for node: {}", _0)]
    CouldNotResolveSource(String),

    #[error("The scope is empty")]
    EmptyScope,

    #[error("Max supported compound identifier length is 2, got: {}", _0)]
    MaxCompoundIdentLengthExceeded(u8),
}

impl From<FindUniqueMatchError> for ResolutionError {
    fn from(value: FindUniqueMatchError) -> Self {
        match value {
            FindUniqueMatchError::Ambiguous(sql_ident) => {
                ResolutionError::AmbiguousIdentifier(sql_ident.to_string())
            }
            FindUniqueMatchError::NotFound(sql_ident) => {
                ResolutionError::NoSuchIdentifier(sql_ident.to_string())
            }
        }
    }
}

impl From<AmbiguousMatchError> for ResolutionError {
    fn from(value: AmbiguousMatchError) -> Self {
        ResolutionError::AmbiguousIdentifier(value.0.to_string())
    }
}
