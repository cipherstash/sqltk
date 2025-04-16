use std::fmt::Debug;

use sqltk_parser::ast::{OneOrManyWithParens, WrappedCollection};

use crate::{Transform, Transformable, Visitable};

impl<'ast, N> Transformable<'ast> for Vec<N>
where
    N: Visitable + Transformable<'ast>,
{
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: Transform<'ast>,
    {
        path.push(self);

        let items = self
            .iter()
            .map(|item| item.apply_transform_with_path(transformer, path))
            .collect::<Result<Vec<N>, T::Error>>()?;

        let transformed = transformer.transform(path, items)?;

        path.pop();

        Ok(transformed)
    }
}

impl<'ast, N> Transformable<'ast> for Option<N>
where
    N: Visitable + Transformable<'ast>,
{
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: Transform<'ast>,
    {
        path.push(self);

        let item = self
            .as_ref()
            .map(|item| item.apply_transform_with_path(transformer, path))
            .transpose()?;

        let transformed = transformer.transform(path, item)?;

        path.pop();

        Ok(transformed)
    }
}

impl<'ast, N> Transformable<'ast> for Box<N>
where
    N: Visitable + Transformable<'ast>,
{
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: Transform<'ast>,
    {
        // NOTE: there is no push/pop of the path for Box<N>.

        let transformed = (**self)
            .apply_transform_with_path(transformer, path)
            .map(Box::new)?;

        // let transformed = transformer.transform(path, item)?;

        Ok(transformed)
    }
}

// The following impls are special cases of sqltk-parser AST nodes (sqltk-codegen cannot handle generics).

impl<'ast, N> Transformable<'ast> for OneOrManyWithParens<N>
where
    N: Visitable + Debug + Transformable<'ast>,
{
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: Transform<'ast>,
    {
        path.push(self);

        let transformed = match self {
            OneOrManyWithParens::One(node) => {
                OneOrManyWithParens::One(node.apply_transform_with_path(transformer, path)?)
            }
            OneOrManyWithParens::Many(nodes) => {
                OneOrManyWithParens::Many(nodes.apply_transform_with_path(transformer, path)?)
            }
        };

        path.pop();

        Ok(transformed)
    }
}

impl<'ast, N> Transformable<'ast> for WrappedCollection<N>
where
    N: Visitable + Transformable<'ast>,
{
    fn apply_transform_with_path<T>(
        &'ast self,
        transformer: &mut T,
        path: &mut crate::NodePath<'ast>,
    ) -> Result<Self, T::Error>
    where
        T: Transform<'ast>,
    {
        path.push(self);

        let transformed = match self {
            WrappedCollection::NoWrapping(nodes) => {
                WrappedCollection::NoWrapping(nodes.apply_transform_with_path(transformer, path)?)
            }
            WrappedCollection::Parentheses(nodes) => {
                WrappedCollection::Parentheses(nodes.apply_transform_with_path(transformer, path)?)
            }
        };

        path.pop();

        Ok(transformed)
    }
}
