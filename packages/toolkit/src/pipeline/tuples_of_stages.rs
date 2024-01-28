use super::stage::Stage;

pub trait TupleOfStages<'ast> {}

impl<'ast, 'scope, A> TupleOfStages<'ast> for (A,) where A: Stage<'ast, 'scope> {}

impl<'ast, 'scope, A, B> TupleOfStages<'ast> for (A, B)
where
    A: Stage<'ast, 'scope>,
    B: Stage<'ast, 'scope>,
{
}

impl<'ast, 'scope, A, B, C> TupleOfStages<'ast> for (A, B, C)
where
    A: Stage<'ast, 'scope>,
    B: Stage<'ast, 'scope>,
    C: Stage<'ast, 'scope>,
{
}

impl<'ast, 'scope, A, B, C, D> TupleOfStages<'ast> for (A, B, C, D)
where
    A: Stage<'ast, 'scope>,
    B: Stage<'ast, 'scope>,
    C: Stage<'ast, 'scope>,
    D: Stage<'ast, 'scope>,
{
}

impl<'ast, 'scope, A, B, C, D, E> TupleOfStages<'ast> for (A, B, C, D, E)
where
    A: Stage<'ast, 'scope>,
    B: Stage<'ast, 'scope>,
    C: Stage<'ast, 'scope>,
    D: Stage<'ast, 'scope>,
    E: Stage<'ast, 'scope>,
{
}
