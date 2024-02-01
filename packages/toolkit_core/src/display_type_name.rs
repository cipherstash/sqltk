/// Newtype and trait to override [`Display`] implementation of `sqlparser` AST
/// node types.

use std::{
    fmt::{Display, Formatter, Result},
    marker::PhantomData,
};

/// Newtype wrapper so that a custom [`std::fmt::Display`] can be implemented
/// for `sqlparser` AST types.
pub struct DisplayTypeName<T>(PhantomData<T>);

pub fn display_type_name_of_value<T>(_: T) -> DisplayTypeName<T> {
    DisplayTypeName(PhantomData)
}

impl<'ast, T> Display for DisplayTypeName<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", tynm::type_name::<T>().replace("&", ""))
    }
}
