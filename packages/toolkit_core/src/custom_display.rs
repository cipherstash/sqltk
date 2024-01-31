/// Newtype and trait to override [`Display`] implementation of `sqlparser` AST
/// node types.

use std::{
    fmt::{Display, Formatter, Result},
    marker::PhantomData,
};

use crate::Visitable;

/// Newtype wrapper so that a custom [`std::fmt::Display`] can be implemented
/// for `sqlparser` AST types.
pub struct DisplayType<T>(PhantomData<T>);

impl<T> Default for DisplayType<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> DisplayType<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<'ast, T: Visitable<'ast>> Display for DisplayType<Box<T>>
where
    DisplayType<T>: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Box<{}>", DisplayType::<T>::new())
    }
}

impl<'ast, T: Visitable<'ast>> Display for DisplayType<Vec<T>>
where
    DisplayType<T>: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Vec<{}>", DisplayType::<T>::new())
    }
}

impl<'ast, T: Visitable<'ast>> Display for DisplayType<Option<T>>
where
    DisplayType<T>: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Option<{}>", DisplayType::<T>::new())
    }
}
