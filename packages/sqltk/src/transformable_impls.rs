use std::any::Any;

use sqlparser::ast::{OneOrManyWithParens, WrappedCollection};

use crate::{Transform, Transformable, Visitable};

impl<'ast, N> Transformable<'ast> for Vec<N>
where
    N: Visitable + Transformable<'ast>,
{
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        path: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: Transform<'ast>,
    {
        path.push(self as &'ast dyn Any);

        let items = self
            .iter()
            .map(|item| item.apply_transform_with_path(transformer, path))
            .collect::<Result<Vec<N>, T::Error>>()?;

        path.pop();

        transformer.transform(items, self, path)
    }
}

impl<'ast, N> Transformable<'ast> for Option<N>
where
    N: Visitable + Transformable<'ast>,
{
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        path: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: Transform<'ast>,
    {
        path.push(self as &'ast dyn Any);

        let item = self
            .as_ref()
            .map(|item| item.apply_transform_with_path(transformer, path))
            .transpose()?;

        path.pop();

        transformer.transform(item, self, path)
    }
}

impl<'ast, N> Transformable<'ast> for Box<N>
where
    N: Visitable + Transformable<'ast>,
{
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        path: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: Transform<'ast>,
    {
        path.push(self as &'ast dyn Any);

        let item = (**self)
            .apply_transform_with_path(transformer, path)
            .map(Box::new)?;

        path.pop();

        transformer.transform(item, self, path)
    }
}

// The following impls are special cases of sqlparser AST nodes (sqltk-codegen cannot handle generics).

impl<'ast, N> Transformable<'ast> for OneOrManyWithParens<N>
where
    N: Visitable + Transformable<'ast>,
{
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        path: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: Transform<'ast>,
    {
        path.push(self as &'ast dyn Any);

        let result = match self {
            OneOrManyWithParens::One(node) => {
                OneOrManyWithParens::One(node.apply_transform_with_path(transformer, path)?)
            }
            OneOrManyWithParens::Many(nodes) => {
                OneOrManyWithParens::Many(nodes.apply_transform_with_path(transformer, path)?)
            }
        };

        path.pop();

        Ok(result)
    }
}

impl<'ast, N> Transformable<'ast> for WrappedCollection<N>
where
    N: Visitable + Transformable<'ast>,
{
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        path: &mut crate::Context<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: Transform<'ast>,
    {
        path.push(self as &'ast dyn Any);

        let result = match self {
            WrappedCollection::NoWrapping(nodes) => {
                WrappedCollection::NoWrapping(nodes.apply_transform_with_path(transformer, path)?)
            }
            WrappedCollection::Parentheses(nodes) => {
                WrappedCollection::Parentheses(nodes.apply_transform_with_path(transformer, path)?)
            }
        };

        path.pop();

        Ok(result)
    }
}
