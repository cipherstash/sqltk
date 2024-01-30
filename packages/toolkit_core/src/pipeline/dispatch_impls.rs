use crate::{pipeline::Pipeline, SqlNode, EnterControlFlow, ExitControlFlow, VisitorDispatch};

use super::{PipelineDispatch, Stage};

impl<'ast, 'scope: 'ast, A> PipelineDispatch<'ast, 'scope> for Pipeline<'ast, 'scope, (A,)> where
    A: Stage<'ast, 'scope>
{
}

impl<'ast, 'scope: 'ast, A> VisitorDispatch<'ast> for Pipeline<'ast, 'scope, (A,)>
where
    A: Stage<'ast, 'scope>,
{
    fn enter(&mut self, sql_node: SqlNode<'ast>) -> EnterControlFlow {
        self.stages.0.enter(sql_node)
    }

    fn exit(&mut self, sql_node: SqlNode<'ast>) -> ExitControlFlow {
        self.stages.0.exit(sql_node)
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
    fn enter(&mut self, sql_node: SqlNode<'ast>) -> EnterControlFlow {
        self.stages.0.enter(sql_node.clone())?;
        self.stages.1.enter(sql_node)
    }

    fn exit(&mut self, sql_node: SqlNode<'ast>) -> ExitControlFlow {
        self.stages.1.exit(sql_node.clone());
        self.stages.0.exit(sql_node)
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
    fn enter(&mut self, sql_node: SqlNode<'ast>) -> EnterControlFlow {
        self.stages.0.enter(sql_node.clone())?;
        self.stages.1.enter(sql_node.clone())?;
        self.stages.2.enter(sql_node)
    }

    fn exit(&mut self, sql_node: SqlNode<'ast>) -> ExitControlFlow {
        self.stages.2.exit(sql_node.clone());
        self.stages.1.exit(sql_node.clone());
        self.stages.0.exit(sql_node)
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
    fn enter(&mut self, sql_node: SqlNode<'ast>) -> EnterControlFlow {
        self.stages.0.enter(sql_node.clone())?;
        self.stages.1.enter(sql_node.clone())?;
        self.stages.2.enter(sql_node.clone())?;
        self.stages.3.enter(sql_node)
    }

    fn exit(&mut self, sql_node: SqlNode<'ast>) -> ExitControlFlow {
        self.stages.3.exit(sql_node.clone());
        self.stages.2.exit(sql_node.clone());
        self.stages.1.exit(sql_node.clone());
        self.stages.0.exit(sql_node)
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
    fn enter(&mut self, sql_node: SqlNode<'ast>) -> EnterControlFlow {
        self.stages.0.enter(sql_node.clone())?;
        self.stages.1.enter(sql_node.clone())?;
        self.stages.2.enter(sql_node.clone())?;
        self.stages.3.enter(sql_node.clone())?;
        self.stages.4.enter(sql_node)
    }

    fn exit(&mut self, sql_node: SqlNode<'ast>) -> ExitControlFlow {
        self.stages.4.exit(sql_node.clone());
        self.stages.3.exit(sql_node.clone());
        self.stages.2.exit(sql_node.clone());
        self.stages.1.exit(sql_node.clone());
        self.stages.0.exit(sql_node)
    }
}
