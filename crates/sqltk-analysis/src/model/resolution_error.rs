//! Errors that can be returned during resolution of identifiers in scope or
//! when attempting to resolve the [`crate::model::sources::Source`] of an
//! [`sqlparser::ast::Expr`].

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

    #[error("Unknown compound identifier: {:?}", _0)]
    NoSuchCompoundIdentifier(String),

    #[error("Tried to add table column that was already in scope: {:?}", _0)]
    AlreadyInScope(String),

    #[error("Something went wrong: {}", _0)]
    InvariantFailed(InvariantFailedError)
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