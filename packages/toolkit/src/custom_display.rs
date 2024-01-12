use std::{fmt::{Display, Formatter, Result}, marker::PhantomData};

use crate::AstNode;

/// Newtype wrapper so that a custom [`std::fmt::Display`] can be implemented
/// for `sqlparser` AST types.
pub struct DisplayType<T>(PhantomData<T>);

impl<T> DisplayType<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

pub fn display_type_from_value<T>(_value: &T) -> DisplayType<T> {
    DisplayType::<T>(PhantomData)
}

impl<'ast, T: AstNode<'ast>> Display for DisplayType<Box<T>> where DisplayType<T>: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Box<{}>", DisplayType::<T>::new())
    }
}

impl<'ast, T: AstNode<'ast>> Display for DisplayType<Vec<T>> where DisplayType<T>: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Vec<{}>", DisplayType::<T>::new())
    }
}

impl<'ast, T: AstNode<'ast>> Display for DisplayType<Option<T>> where DisplayType<T>: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Option<{}>", DisplayType::<T>::new())
    }
}