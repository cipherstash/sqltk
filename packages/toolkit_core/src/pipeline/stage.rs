use crate::{pipeline::Scope, UnknownItemError, VisitorDispatch};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum StageInitError {
    #[error("Stage initialization failed due to: {0}")]
    UnknownItem(#[from] UnknownItemError)
}

pub trait Stage<'ast, 'scope>
where
    Self: VisitorDispatch<'ast> + Sized,
{
    #[allow(unused_variables)]
    fn init_enter(scope: &mut impl Scope<'scope>) -> Result<(), StageInitError> {
        Ok(())
    }

    fn init_exit(scope: &mut impl Scope<'scope>) -> Result<Self, StageInitError>;
}
