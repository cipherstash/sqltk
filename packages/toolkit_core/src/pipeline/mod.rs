//! A [`Pipeline`] composes multiple [`crate::VisitorDispatch`] implementations
//! which facilitates the building of arbitrarily sophisticated AST analysis and
//! transformation workflows from small unit-testable pieces.

mod accessors;

use std::error::Error;

use crate::{EnterControlFlow, ExitControlFlow, Navigation, SqlNode, Visitable, VisitorDispatch};

pub use accessors::*;

mod private {
    pub trait Sealed {}
}

/// A `Pipeline` is a composition of `VisitorDispatch` implementations (called
/// "stages") that colloborate to process a `sqlparser` AST to produce a
/// specific output.
///
/// Pipelines therefore facilitate composition of small, decoupled, unit
/// testable pieces AST analysis functionality.
///
/// Each stage can observe zero or more values produced by other stages. The
/// underlying mechanism for datasharing is not mandated by the `Pipeline`
/// trait, but typically stages will share data via the "accessors"
/// [`crate::ReadOnly`] and [`crate::ReadWrite`] which are wrappers around
/// an `Rc<RefCell<T>>` which provides opt-in encapsulation.
///
/// A `Pipeline` implementation must define the associated `Output` type, which
/// is the final result type of a successful execution.
///
/// ## Execution sequence
///
/// Pipelines are built via [`PipelineBuilder`] and stages are added by
/// [`PipelineBuilder::add_stage`].
///
/// The order of calls to `add_stage` determines the execution order.
///
/// Imagine there are three stages that were added in this order: `stage0`,
/// `stage1`, `stage2`.
///
/// When an AST node is entered, stages are executed in the following order:
///
/// `stage0.enter(..)` then `stage1.enter(..)` then `stage2.enter(..)`.
///
/// Exiting an an AST node invokes the stages in reverse:
///
/// `stage2.exit(..)` then `stage1.exit(..)` then `stage0.exit(..)`.
pub trait Pipeline: private::Sealed
where
    Self: Sized,
{
    /// The value produced from a successful execution.
    type Output;

    /// Executes and consumes the pipeline, returning `Ok(Self::Output)` or
    /// `Err(Box<dyn Error>)`.
    fn execute<'ast, N>(self, node: &'ast N) -> Result<Self::Output, Box<dyn Error>>
    where
        N: Visitable,
        &'ast N: Into<SqlNode<'ast>>;
}

/// Helper for building a `Pipeline` implementation from an arbitrary number of
/// [`VisitorDispatch`] implementations.
pub struct PipelineBuilder<Output> {
    stages: Vec<Box<dyn VisitorDispatch>>,
    output: Output,
}

impl Default for PipelineBuilder<()> {
    fn default() -> Self {
        Self::new()
    }
}

impl PipelineBuilder<()> {
    /// Creates a new `PipelineBuilder`
    pub fn new() -> Self {
        PipelineBuilder {
            stages: Vec::new(),
            output: (),
        }
    }

    /// Adds a new stage. The order of calls to `add_stage` determines the order
    /// of invocation of stages during AST traversal.  See the [`Pipeline`]
    /// trait documentation for more info.
    pub fn add_stage<Stage>(self, stage: Stage) -> Self
    where
        Stage: 'static + VisitorDispatch,
    {
        let mut me = self;
        me.stages.push(Box::new(stage));
        me
    }

    /// Sets the output value (and type) of the pipeline.
    pub fn output<Output>(self, output: Output) -> PipelineBuilder<Output> {
        PipelineBuilder {
            stages: self.stages,
            output,
        }
    }
}

impl<Output> PipelineBuilder<Output> {
    /// Builds a `Pipeline` from the current configuration stored in this builder.
    ///
    /// Note that this method does not yet check for pointless configuration
    /// (such as forgetting to set the output type, or forgetting to add
    /// stages).
    ///
    /// In the future it might be changed to return a `Result` (or prove
    /// correctness via the type system).
    pub fn build(self) -> impl Pipeline<Output = Output> {
        ConcretePipeline::new(self.stages, self.output)
    }
}

/// Concrete implementation of a `Pipeline`.
///
/// When inherent associated types land in stable Rust the trait can be done
/// away with and this type will be renamed to `Pipeline`.
struct ConcretePipeline<Output> {
    output: Output,
    stages: Vec<Box<dyn VisitorDispatch>>,
}

impl<Output> ConcretePipeline<Output> {
    /// Takes ownership if the stages and output accessor and returns a new
    /// `ConcretePipeline`.
    fn new(stages: Vec<Box<dyn VisitorDispatch>>, output: Output) -> Self {
        Self { output, stages }
    }
}

impl<Output> private::Sealed for ConcretePipeline<Output> {}

impl<Output> Pipeline for ConcretePipeline<Output> {
    type Output = Output;

    fn execute<'ast, N: Visitable>(self, node: &'ast N) -> Result<Self::Output, Box<dyn Error>>
    where
        N: Visitable,
        &'ast N: Into<SqlNode<'ast>>,
    {
        let mut dispatcher = PipelineDispatcher {
            stages: self.stages,
        };
        node.accept(&mut dispatcher);

        Ok(self.output)
    }
}

/// This struct exists so that the `Pipeline` trait itself does not implement
/// `VisitorDispatch`, which means the [`Pipeline::execute`] method can
/// completely encapsulate the pipeline execution which obtains the output and
/// consumes the pipeline.
struct PipelineDispatcher {
    stages: Vec<Box<dyn VisitorDispatch>>,
}

// TODO: error handling (need to change the Visitor trait signatures to enable
// error returns first)
impl VisitorDispatch for PipelineDispatcher {
    fn enter<'subj, 'ast>(&'subj mut self, node: SqlNode<'ast>) -> EnterControlFlow
    where
        'ast: 'subj,
    {
        for stage in self.stages.iter_mut() {
            let _ = stage.enter(node.clone());
        }
        EnterControlFlow::Continue(Navigation::Visit)
    }

    fn exit<'subj, 'ast>(&mut self, node: SqlNode<'ast>) -> ExitControlFlow
    where
        'ast: 'subj,
    {
        for stage in self.stages.iter_mut().rev() {
            let _ = stage.exit(node.clone());
        }
        ExitControlFlow::Continue(())
    }
}
