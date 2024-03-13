//! A [`Pipeline`] composes multiple [`crate::VisitorDispatch`] implementations
//! which facilitates the building of arbitrarily sophisticated AST analysis and
//! transformation workflows from small unit-testable pieces.

use std::{error::Error, mem};

use crate::{
    Enter, EnterControlFlow, Exit, ExitControlFlow, SqlNode, Visitable, VisitorDispatch,
};

/// A `Pipeline` is a composition of `VisitorDispatch` implementations (called
/// "stages") that collaborate to process a `sqlparser` AST to produce a
/// specific output.
///
/// Pipelines therefore facilitate composition of small, decoupled, unit
/// testable pieces AST analysis functionality.
///
/// A `Pipeline` implementation must define the associated `Output` type, which
/// is the final result type of a successful execution.
pub trait Pipeline<'out, 'ast, Context> {
    type Output: 'out;

    fn execute<N>(&self, node: &'ast N) -> Result<Self::Output, Box<dyn Error>>
    where
        N: Visitable<'ast>,
        &'ast N: Into<SqlNode<'ast>>;
}

pub trait PipelineDispatchable<'ast, Context>: VisitorDispatch<'ast> + AsMut<Context> {
    fn enter_with_swapped_context(&mut self, context: &mut Context, node: SqlNode<'ast>) -> EnterControlFlow;
    fn exit_with_swapped_context(&mut self, context: &mut Context, node: SqlNode<'ast>) -> ExitControlFlow;
}

impl<'ast, Context, T> PipelineDispatchable<'ast, Context> for T where T: VisitorDispatch<'ast> + AsMut<Context> {
    fn enter_with_swapped_context(
        &mut self,
        context: &mut Context,
        node: SqlNode<'ast>,
    ) -> EnterControlFlow {
        mem::swap(self.as_mut(), context);
        let result = VisitorDispatch::enter(self, node.into());
        mem::swap(context, self.as_mut());
        result
    }

    fn exit_with_swapped_context(
        &mut self,
        context: &mut Context,
        node: SqlNode<'ast>,
    ) -> ExitControlFlow {
        mem::swap(self.as_mut(), context);
        let result = VisitorDispatch::exit(self, node.into());
        mem::swap(context, self.as_mut());
        result
    }
}

pub struct PipelineStages<'c, 'ast, Context> {
    stages: Vec<Box<dyn PipelineDispatchable<'ast, Context>>>,
    context: &'c mut Context,
}

impl<'c, 'ast, Context> PipelineStages<'c, 'ast, Context> {
    pub fn new(context: &'c mut Context, stages: Vec<Box<dyn PipelineDispatchable<'ast, Context>>>) -> Self {
        Self { stages, context }
    }
}

impl<'c, 'ast, Context> VisitorDispatch<'ast> for PipelineStages<'c, 'ast, Context> {
    fn enter(&mut self, node: SqlNode<'ast>) -> EnterControlFlow {
        for stage in self.stages.iter_mut() {
            stage.enter_with_swapped_context(&mut self.context, node.clone())?;
        }
        Enter::visit()
    }

    fn exit(&mut self, node: SqlNode<'ast>) -> ExitControlFlow {
        for stage in self.stages.iter_mut().rev() {
            stage.exit_with_swapped_context(&mut self.context, node.clone())?;
        }
        Exit::normal()
    }
}
