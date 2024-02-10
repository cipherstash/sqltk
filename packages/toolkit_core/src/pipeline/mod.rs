//! # Pipelines
//!
//! A [`Pipeline`] composes multiple [`crate::VisitorDispatch`] implementations
//! which facilitates the building of arbitrarily sophisticated AST analysis and
//! transformation workflows from small unit-testable pieces.
//!
//! All `Pipeline` implementations are also [`VisitorDispatch`] implementations
//! so that they can be passed to [`crate::Visitable::accept`].

mod scope;
mod stage;

pub use scope::*;
pub use stage::*;

use crate::Visitable;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PipelineInitError {
    #[error("Failed to initialize Pipeline due to: {0}")]
    StageInitError(#[from] StageInitError)
}

pub trait Pipeline<'ast>
where
    Self: Sized,
{
    fn new(scope: RootScope) -> Result<Self, PipelineInitError>;

    // TODO: error type
    fn execute<N: Visitable<'ast>>(self, node: &'ast N) -> Result<RootScope, RootScope>;
}
