use sqlparser::ast::{OneOrManyWithParens, WrappedCollection};

use crate::{Transform, Transformable, Visitable};

impl<'ast, N> Transformable<'ast> for Vec<N>
where
    N: Visitable + Transformable<'ast>,
{
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: Transform<'ast>,
    {
        let items = self
            .iter()
            .map(|item| item.apply_transform(transformer))
            .collect::<Result<Vec<N>, T::Error>>()?;

        transformer.transform(self, items)
    }
}

impl<'ast, N> Transformable<'ast> for Option<N>
where
    N: Visitable + Transformable<'ast>,
{
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: Transform<'ast>,
    {
        let item = self
            .as_ref()
            .map(|item| item.apply_transform(transformer))
            .transpose()?;

        transformer.transform(self, item)
    }
}

impl<'ast, N> Transformable<'ast> for Box<N>
where
    N: Visitable + Transformable<'ast>,
{
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: Transform<'ast>,
    {
        let item = (**self).apply_transform(transformer).map(Box::new)?;
        transformer.transform(self, item)
    }
}

// The following impls are special cases of sqlparser AST nodes (sqltk-codegen cannot handle generics).

impl<'ast, N> Transformable<'ast> for OneOrManyWithParens<N>
where
    N: Visitable + Transformable<'ast>,
{
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: Transform<'ast>,
    {
        match self {
            OneOrManyWithParens::One(node) => {
                Ok(OneOrManyWithParens::One(node.apply_transform(transformer)?))
            }
            OneOrManyWithParens::Many(nodes) => Ok(OneOrManyWithParens::Many(
                nodes.apply_transform(transformer)?,
            )),
        }
    }
}

impl<'ast, N> Transformable<'ast> for WrappedCollection<N>
where
    N: Visitable + Transformable<'ast>,
{
    fn apply_transform<T>(&'ast self, transformer: &mut T) -> Result<Self, T::Error>
    where
        T: Transform<'ast>,
    {
        match self {
            WrappedCollection::NoWrapping(nodes) => Ok(WrappedCollection::NoWrapping(
                nodes.apply_transform(transformer)?,
            )),
            WrappedCollection::Parentheses(nodes) => Ok(WrappedCollection::Parentheses(
                nodes.apply_transform(transformer)?,
            )),
        }
    }
}
