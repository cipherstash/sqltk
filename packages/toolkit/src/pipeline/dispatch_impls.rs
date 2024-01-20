use crate::{
    pipeline::Pipeline, AstNode, ConcreteNode, Condition, NodeSupport, VisitorControlFlow,
    VisitorDispatch,
};

use super::{PipelineDispatch, Stage, TupleOfStages};

impl<'ast, 'scope: 'ast, Stages: TupleOfStages<'ast>, N: AstNode<'ast>> NodeSupport<N>
    for Pipeline<'ast, 'scope, Stages>
{
    type Supported = Condition<true>;
}

impl<'ast, 'scope: 'ast, A> PipelineDispatch<'ast, 'scope> for Pipeline<'ast, 'scope, (A,)> where
    A: Stage<'ast, 'scope>
{
}

impl<'ast, 'scope: 'ast, A> VisitorDispatch<'ast> for Pipeline<'ast, 'scope, (A,)>
where
    A: Stage<'ast, 'scope>,
{
    fn dispatch_enter(&mut self, concrete_node: ConcreteNode<'ast>) -> VisitorControlFlow {
        self.stages.0.dispatch_enter(concrete_node)
    }

    fn dispatch_exit(&mut self, concrete_node: ConcreteNode<'ast>) -> VisitorControlFlow {
        self.stages.0.dispatch_exit(concrete_node)
    }
}
impl<'ast, 'scope: 'ast, A, B> PipelineDispatch<'ast, 'scope> for Pipeline<'ast, 'scope, (A, B)>
where
    A: Stage<'ast, 'scope>,
    B: Stage<'ast, 'scope>,
{
}

impl<'ast, 'scope: 'ast, A, B> VisitorDispatch<'ast> for Pipeline<'ast, 'scope, (A, B)>
where
    A: Stage<'ast, 'scope>,
    B: Stage<'ast, 'scope>,
{
    fn dispatch_enter(&mut self, concrete_node: ConcreteNode<'ast>) -> VisitorControlFlow {
        self.stages.0.dispatch_enter(concrete_node.clone())?;
        self.stages.1.dispatch_enter(concrete_node)
    }

    fn dispatch_exit(&mut self, concrete_node: ConcreteNode<'ast>) -> VisitorControlFlow {
        self.stages.1.dispatch_exit(concrete_node.clone());
        self.stages.0.dispatch_exit(concrete_node)
    }
}
impl<'ast, 'scope: 'ast, A, B, C> PipelineDispatch<'ast, 'scope>
    for Pipeline<'ast, 'scope, (A, B, C)>
where
    A: Stage<'ast, 'scope>,
    B: Stage<'ast, 'scope>,
    C: Stage<'ast, 'scope>,
{
}

impl<'ast, 'scope: 'ast, A, B, C> VisitorDispatch<'ast> for Pipeline<'ast, 'scope, (A, B, C)>
where
    A: Stage<'ast, 'scope>,
    B: Stage<'ast, 'scope>,
    C: Stage<'ast, 'scope>,
{
    fn dispatch_enter(&mut self, concrete_node: ConcreteNode<'ast>) -> VisitorControlFlow {
        self.stages.0.dispatch_enter(concrete_node.clone())?;
        self.stages.1.dispatch_enter(concrete_node.clone())?;
        self.stages.2.dispatch_enter(concrete_node)
    }

    fn dispatch_exit(&mut self, concrete_node: ConcreteNode<'ast>) -> VisitorControlFlow {
        self.stages.2.dispatch_exit(concrete_node.clone());
        self.stages.1.dispatch_exit(concrete_node.clone());
        self.stages.0.dispatch_exit(concrete_node)
    }
}

impl<'ast, 'scope: 'ast, A, B, C, D> PipelineDispatch<'ast, 'scope>
    for Pipeline<'ast, 'scope, (A, B, C, D)>
where
    A: Stage<'ast, 'scope>,
    B: Stage<'ast, 'scope>,
    C: Stage<'ast, 'scope>,
    D: Stage<'ast, 'scope>,
{
}

impl<'ast, 'scope: 'ast, A, B, C, D> VisitorDispatch<'ast> for Pipeline<'ast, 'scope, (A, B, C, D)>
where
    A: Stage<'ast, 'scope>,
    B: Stage<'ast, 'scope>,
    C: Stage<'ast, 'scope>,
    D: Stage<'ast, 'scope>,
{
    fn dispatch_enter(&mut self, concrete_node: ConcreteNode<'ast>) -> VisitorControlFlow {
        self.stages.0.dispatch_enter(concrete_node.clone())?;
        self.stages.1.dispatch_enter(concrete_node.clone())?;
        self.stages.2.dispatch_enter(concrete_node.clone())?;
        self.stages.3.dispatch_enter(concrete_node)
    }

    fn dispatch_exit(&mut self, concrete_node: ConcreteNode<'ast>) -> VisitorControlFlow {
        self.stages.3.dispatch_exit(concrete_node.clone());
        self.stages.2.dispatch_exit(concrete_node.clone());
        self.stages.1.dispatch_exit(concrete_node.clone());
        self.stages.0.dispatch_exit(concrete_node)
    }
}

impl<'ast, 'scope: 'ast, A, B, C, D, E> PipelineDispatch<'ast, 'scope>
    for Pipeline<'ast, 'scope, (A, B, C, D, E)>
where
    A: Stage<'ast, 'scope>,
    B: Stage<'ast, 'scope>,
    C: Stage<'ast, 'scope>,
    D: Stage<'ast, 'scope>,
    E: Stage<'ast, 'scope>,
{
}

impl<'ast, 'scope: 'ast, A, B, C, D, E> VisitorDispatch<'ast>
    for Pipeline<'ast, 'scope, (A, B, C, D, E)>
where
    A: Stage<'ast, 'scope>,
    B: Stage<'ast, 'scope>,
    C: Stage<'ast, 'scope>,
    D: Stage<'ast, 'scope>,
    E: Stage<'ast, 'scope>,
{
    fn dispatch_enter(&mut self, concrete_node: ConcreteNode<'ast>) -> VisitorControlFlow {
        self.stages.0.dispatch_enter(concrete_node.clone())?;
        self.stages.1.dispatch_enter(concrete_node.clone())?;
        self.stages.2.dispatch_enter(concrete_node.clone())?;
        self.stages.3.dispatch_enter(concrete_node.clone())?;
        self.stages.4.dispatch_enter(concrete_node)
    }

    fn dispatch_exit(&mut self, concrete_node: ConcreteNode<'ast>) -> VisitorControlFlow {
        self.stages.4.dispatch_exit(concrete_node.clone());
        self.stages.3.dispatch_exit(concrete_node.clone());
        self.stages.2.dispatch_exit(concrete_node.clone());
        self.stages.1.dispatch_exit(concrete_node.clone());
        self.stages.0.dispatch_exit(concrete_node)
    }
}
