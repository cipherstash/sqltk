//! A [`Pipeline`] composes multiple [`crate::VisitorDispatch`] implementations
//! which facilitates the building of arbitrarily sophisticated AST analysis and
//! transformation workflows from small unit-testable pieces.

use std::error::Error;

use crate::{Flow, SqlNode, Visitable, VisitorDispatch};

/// A `Pipeline` is a composition of `VisitorDispatch` implementations (called
/// "stages") that collaborate to process a `sqlparser` AST to produce a
/// specific output.
///
/// Pipelines therefore facilitate composition of small, decoupled, unit
/// testable pieces AST analysis functionality.
///
/// A `Pipeline` implementation must define the associated `Output` type, which
/// is the final result type of a successful execution.
pub trait Pipeline<'out, 'ast, State> {
    type Output: 'out;

    fn execute<N>(&self, node: &'ast N) -> Result<Self::Output, PipelineError<State>>
    where
        N: Visitable<'ast>,
        &'ast N: Into<SqlNode<'ast>>;
}

#[derive(thiserror::Error, Debug)]
pub enum PipelineError<State> {
    #[error("Error from Pipeline stage")]
    Error(Box<dyn Error>, State),
}

impl<'v, 'ast, State> VisitorDispatch<'ast, State>
    for &'v [Box<dyn VisitorDispatch<'ast, State>>]
{
    fn enter(
        &self,
        node: SqlNode<'ast>,
        mut state: State,
    ) -> crate::VisitorControlFlow<State> {
        for dispatch in self.iter() {
            state = dispatch.enter(node.clone(), state)?;
        }
        Flow::cont(state)
    }

    fn exit(
        &self,
        node: SqlNode<'ast>,
        mut state: State,
    ) -> crate::VisitorControlFlow<State> {
        for dispatch in self.iter().rev() {
            state = dispatch.exit(node.clone(), state)?;
        }
        Flow::cont(state)
    }
}

// pub trait PipelineDispatchable<'ast, State: 'ast>: VisitorDispatch<'ast, State> + AsMut<State> {
//     fn enter_with_swapped_context(&mut self, node: SqlNode<'ast>, state: State) -> VisitorControlFlow<State>;
//     fn exit_with_swapped_context(&mut self, node: SqlNode<'ast>, state: State) -> VisitorControlFlow<State>;
// }

// impl<'ast, State: 'ast, T> PipelineDispatchable<'ast, State> for T where T: VisitorDispatch<'ast, State> + AsMut<State> {
//     fn enter_with_swapped_context(
//         &mut self,
//         node: SqlNode<'ast>,
//         state: State,
//     ) -> VisitorControlFlow<State> {
//         // mem::swap(self.as_mut(), state);
//         let result = VisitorDispatch::enter(self, node.into(), state);
//         // mem::swap(state, self.as_mut());
//         result
//     }

//     fn exit_with_swapped_context(
//         &mut self,
//         node: SqlNode<'ast>,
//         state: State,
//     ) -> VisitorControlFlow<State> {
//         // mem::swap(self.as_mut(), state);
//         let result = VisitorDispatch::exit(self, node.into(), state);
//         // mem::swap(state, self.as_mut());
//         result
//     }
// }

// pub struct PipelineStages<'c, 'ast, State> {
//     stages: Vec<Box<dyn PipelineDispatchable<'ast, State>>>,
//     state: &'c mut State,
// }

// impl<'c, 'ast, State> PipelineStages<'c, 'ast, State> {
//     pub fn new(state: &'c mut State, stages: Vec<Box<dyn PipelineDispatchable<'ast, State>>>) -> Self {
//         Self { stages, state }
//     }
// }

// impl<'c, 'ast, State: 'ast> VisitorDispatch<'ast, State> for PipelineStages<'c, 'ast, State> {
//     fn enter(&self, node: SqlNode<'ast>, state: State) -> VisitorControlFlow<State> {
//         for stage in self.stages.iter_mut() {
//             stage.enter_with_swapped_context(node.clone(), state)?;
//         }
//         Flow::cont(state)
//     }

//     fn exit(&self, node: SqlNode<'ast>, state: State) -> VisitorControlFlow<State> {
//         for stage in self.stages.iter_mut().rev() {
//             stage.exit_with_swapped_context(node.clone(), state)?;
//         }
//         Flow::cont(state)
//     }
// }
