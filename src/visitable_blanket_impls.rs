use sqlparser::ast::{OneOrManyWithParens, WrappedCollection};
use visitor_helper::visit;

use crate::*;

impl<N> Visitable for Vec<N>
where
    N: Visitable,
{
    fn accept<'ast, V: Visitor<'ast>>(&'ast self, visitor: &mut V) -> ControlFlow<Break<V::Error>> {
        if self.is_empty() {
            ControlFlow::Continue(())
        } else {
            visit(self, visitor, |visitor| {
                for child in self.iter() {
                    child.accept(visitor)?;
                }
                ControlFlow::Continue(())
            })
        }
    }
}

/// A `Vec<N: Visitable>` is semantically interesting.
impl<N> Semantic for Vec<N> where N: Visitable {}

/// Does not implement `Semantic` because `N` is the semantically interesting type.
impl<N> Visitable for Box<N>
where
    N: Visitable,
{
    fn accept<'ast, V: Visitor<'ast>>(&'ast self, visitor: &mut V) -> ControlFlow<Break<V::Error>> {
        (**self).accept(visitor)
    }
}

/// Does not implement `Semantic` because `N` is the semantically interesting type.
impl<N> Visitable for Option<N>
where
    N: Visitable,
{
    fn accept<'ast, V: Visitor<'ast>>(&'ast self, visitor: &mut V) -> ControlFlow<Break<V::Error>> {
        if self.is_none() {
            ControlFlow::Continue(())
        } else {
            visit(self, visitor, |visitor| match self {
                Some(child) => child.accept(visitor),
                None => ControlFlow::Continue(()),
            })
        }
    }
}

/// Manual implementaton because sqltk-codegen cannot handle generics.
/// Does not implement `Semantic`.
impl<N> Visitable for OneOrManyWithParens<N>
where
    N: 'static + Visitable,
{
    fn accept<'ast, V: Visitor<'ast>>(&'ast self, visitor: &mut V) -> ControlFlow<Break<V::Error>> {
        match self {
            OneOrManyWithParens::One(node) => node.accept(visitor),
            OneOrManyWithParens::Many(nodes) => nodes.accept(visitor),
        }
    }
}

/// Manual implementaton because sqltk-codegen cannot handle generics.
/// Does not implement `Semantic`.
impl<N> Visitable for WrappedCollection<N>
where
    N: 'static + Visitable,
{
    fn accept<'ast, V: Visitor<'ast>>(&'ast self, visitor: &mut V) -> ControlFlow<Break<V::Error>> {
        match self {
            WrappedCollection::NoWrapping(nodes) => nodes.accept(visitor),
            WrappedCollection::Parentheses(nodes) => nodes.accept(visitor),
        }
    }
}