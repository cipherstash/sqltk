use crate::{ApplyTransform, Transform};

include!(concat!(
    env!("OUT_DIR"),
    "/generated_apply_transform_impls.rs"
));

impl<'ast, N> ApplyTransform<'ast> for Vec<N>
where
    N: 'static + ApplyTransform<'ast>,
{
    fn apply_transform<T>(
        &'ast self,
        transformer: &T,
        context: &T::Context,
    ) -> Result<Self, T::Error>
    where
        T: Transform<'ast>,
    {
        transformer.transform(
            self,
            self.iter()
                .map(|item| item.apply_transform(transformer, context))
                .collect::<Result<Vec<N>, T::Error>>()?,
            context,
        )
    }
}

impl<'ast, N> ApplyTransform<'ast> for Option<N>
where
    N: 'static + ApplyTransform<'ast>,
{
    fn apply_transform<T>(
        &'ast self,
        transformer: &T,
        context: &T::Context,
    ) -> Result<Self, T::Error>
    where
        T: Transform<'ast>,
    {
        transformer.transform(
            self,
            self.as_ref()
                .map(|item| item.apply_transform(transformer, context))
                .transpose()?,
            context,
        )
    }
}

impl<'ast, N> ApplyTransform<'ast> for Box<N>
where
    N: 'static + ApplyTransform<'ast>,
{
    fn apply_transform<T>(
        &'ast self,
        transformer: &T,
        context: &T::Context,
    ) -> Result<Self, T::Error>
    where
        T: Transform<'ast>,
    {
        transformer.transform(
            self,
            (**self)
                .apply_transform(transformer, context)
                .map(Box::new)?,
            context,
        )
    }
}
