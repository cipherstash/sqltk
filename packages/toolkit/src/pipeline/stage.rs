use crate::{pipeline::Scope, VisitorDispatch};

pub struct InitializeError;

#[allow(unused_variables)]
pub trait Stage<'ast, 'scope>
where
    Self: VisitorDispatch<'ast> + Sized,
{
    fn init_enter(scope: &mut impl Scope<'scope>) -> Result<(), InitializeError> {
        Ok(())
    }

    fn init_exit(scope: &mut impl Scope<'scope>) -> Result<Self, InitializeError>;
}
