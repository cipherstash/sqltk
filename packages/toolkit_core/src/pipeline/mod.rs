//! # Pipelines
//!
//! A [`Pipeline`] allows composition of multiple [`crate::Visitor`]
//! implementations to facilitate build sophisticated AST analysis and
//! transformation workflows from small, isolated and unit testable pieces.
//!
//! All `Pipeline` implementations are also [`VisitorDispatch`] implementations
//! so that they can be passed to [`crate::Visitable::accept`].
//!
//! ## Building a pipeline
//!
//! A `Pipeline` is created by calling [`build`] which is passed a tuple of
//! [`Stage`] implementations as a generic parameter.
//!

mod dispatch_impls;
mod pipeline_impls;
mod scope;
mod stage;
mod tuples_of_stages;

pub use scope::*;
pub use stage::*;
pub use tuples_of_stages::*;

use crate::VisitorDispatch;
use std::marker::PhantomData;

pub trait PipelineInit<'ast, 'scope: 'ast>
where
    Self: Sized,
{
    fn new(scope: RootScope) -> Result<Self, InitializeError>;
}

pub trait PipelineDispatch<'ast, 'scope: 'ast>: VisitorDispatch<'ast> {}

pub struct Pipeline<'ast, 'scope: 'ast, Stages: TupleOfStages<'ast>> {
    stages: Stages,
    scope: RootScope,
    _ast: PhantomData<&'ast ()>,
    _scope: PhantomData<&'scope ()>,
}

impl<'ast, 'scope: 'ast, Stages: TupleOfStages<'ast>> Pipeline<'ast, 'scope, Stages> {
    pub fn get<T: 'static>(&self) -> Result<ReadOnly<T>, UnknownItemError> {
        self.scope.get::<T>()
    }
}

pub fn build<'ast, 'scope: 'ast, Stages: TupleOfStages<'ast>>(
    scope: RootScope,
) -> Result<Pipeline<'ast, 'scope, Stages>, InitializeError>
where
    Pipeline<'ast, 'scope, Stages>: PipelineInit<'ast, 'scope> + PipelineDispatch<'ast, 'scope>,
{
    Pipeline::<'ast, 'scope, Stages>::new(scope)
}
