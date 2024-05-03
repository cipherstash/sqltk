//! Errors that can be returned during resolution of identifiers in scope or
//! when attempting to resolve the [`crate::model::sources::Source`] of an
//! [`sqlparser::ast::Expr`].

use sqlparser::ast::{Expr, Ident, Query, SetExpr};

use crate::{
    annotations::ExpectedAnnotationError, projection_annotation::ProjectionAnnotation,
    source_annotation::SourceAnnotation,
};

/// Error that can be returned when either:
/// - trying to resolve identifiers in scope, or
/// - trying to add the same table/view/aliased item to scope more than once
/// - some internal invariant failed
///
/// Regarding failed invariants: they 100% represent internal bugs and therefore
/// should not be part of the API (invariant failures should trigger a panic)
/// and those variants will be removed when the `sqltk-analysis` stabilises.
#[derive(Debug, thiserror::Error)]
pub enum ResolutionError {
    #[error("Unknown identifier: {:?}", _0)]
    NoSuchIdentifier(String),

    #[error("No such relation: {:?}", _0)]
    NoSuchRelation(String),

    #[error("Unknown compound identifier: {:?}", _0)]
    NoSuchCompoundIdentifier(String),

    #[error("Tried to add table column that was already in scope: {:?}", _0)]
    AlreadyInScope(String),

    #[error("Something went wrong: {}", _0)]
    InvariantFailed(#[from] InvariantFailedError),

    #[error("Missing source annotation: {}", _0)]
    ExpectedSourceAnnotation(#[from] ExpectedAnnotationError<SourceAnnotation>),

    #[error("Missing projection annotation: {}", _0)]
    ExpectedProjectionAnnotation(#[from] ExpectedAnnotationError<ProjectionAnnotation>),

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
    UnsupportedCompoundIdentifierLength(Vec<Ident>),

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
}

#[derive(Debug, thiserror::Error)]
pub enum InvariantFailedError {
    #[error("Could not resolve source for node: {}", _0)]
    CouldNotResolveSource(String),

    #[error("The scope is empty")]
    EmptyScope,

    #[error("Max supported compound identifier length is 2, got: {}", _0)]
    MaxCompoundIdentLengthExceeded(u8),
}
