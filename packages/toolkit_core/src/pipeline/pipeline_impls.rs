use std::marker::PhantomData;

use super::{InitializeError, Pipeline, PipelineInit, RootScope, Stage};

impl<'ast, 'scope: 'ast, A> PipelineInit<'ast, 'scope> for Pipeline<'ast, 'scope, (A,)>
where
    A: Stage<'ast, 'scope>,
{
    fn new(scope: RootScope) -> Result<Self, InitializeError> {
        let mut scope = scope;
        A::init_enter(&mut scope)?;
        let a = A::init_exit(&mut scope)?;
        Ok(Self {
            stages: (a,),
            scope,
            _ast: PhantomData,
            _scope: PhantomData,
        })
    }
}

impl<'ast, 'scope: 'ast, A, B> PipelineInit<'ast, 'scope> for Pipeline<'ast, 'scope, (A, B)>
where
    A: Stage<'ast, 'scope>,
    B: Stage<'ast, 'scope>,
{
    fn new(scope: RootScope) -> Result<Self, InitializeError> {
        let mut scope = scope;
        A::init_enter(&mut scope)?;
        B::init_enter(&mut scope)?;
        let b = B::init_exit(&mut scope)?;
        let a = A::init_exit(&mut scope)?;
        Ok(Self {
            stages: (a, b),
            scope,
            _ast: PhantomData,
            _scope: PhantomData,
        })
    }
}
