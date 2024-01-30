use super::{polymap::PolyMap, IntoAccessor, StoredItem};
use std::fmt::Debug;

pub trait ScopeItems {
    type Unpack;

    fn resolve(items: &PolyMap) -> Self::Unpack;
}

impl ScopeItems for () {
    type Unpack = ();

    fn resolve(_: &PolyMap) -> Self::Unpack {}
}

impl<A: 'static, SA: IntoAccessor<Inner = A>> ScopeItems for (SA, ()) {
    type Unpack = (Accessor<SA>,);

    fn resolve(items: &PolyMap) -> Self::Unpack {
        (infallible(
            items.get::<StoredItem<A>>().map(SA::accessor_for),
        ),)
    }
}

impl<A: 'static, B: 'static, SA: IntoAccessor<Inner = A>, SB: IntoAccessor<Inner = B>> ScopeItems
    for (SA, (SB, ()))
{
    type Unpack = (Accessor<SB>, Accessor<SA>);

    fn resolve(items: &PolyMap) -> Self::Unpack {
        (
            infallible(items.get::<StoredItem<B>>().map(SB::accessor_for)),
            infallible(items.get::<StoredItem<A>>().map(SA::accessor_for)),
        )
    }
}

impl<
        A: 'static,
        B: 'static,
        C: 'static,
        SA: IntoAccessor<Inner = A>,
        SB: IntoAccessor<Inner = B>,
        SC: IntoAccessor<Inner = C>,
    > ScopeItems for (SA, (SB, (SC, ())))
{
    type Unpack = (Accessor<SC>, Accessor<SB>, Accessor<SA>);

    fn resolve(items: &PolyMap) -> Self::Unpack {
        (
            infallible(items.get::<StoredItem<C>>().map(SC::accessor_for)),
            infallible(items.get::<StoredItem<B>>().map(SB::accessor_for)),
            infallible(items.get::<StoredItem<A>>().map(SA::accessor_for)),
        )
    }
}

impl<
        A: 'static,
        B: 'static,
        C: 'static,
        D: 'static,
        SA: IntoAccessor<Inner = A>,
        SB: IntoAccessor<Inner = B>,
        SC: IntoAccessor<Inner = C>,
        SD: IntoAccessor<Inner = D>,
    > ScopeItems for (SA, (SB, (SC, (SD, ()))))
{
    type Unpack = (Accessor<SD>, Accessor<SC>, Accessor<SB>, Accessor<SA>);

    fn resolve(items: &PolyMap) -> Self::Unpack {
        (
            infallible(items.get::<StoredItem<D>>().map(SD::accessor_for)),
            infallible(items.get::<StoredItem<C>>().map(SC::accessor_for)),
            infallible(items.get::<StoredItem<B>>().map(SB::accessor_for)),
            infallible(items.get::<StoredItem<A>>().map(SA::accessor_for)),
        )
    }
}

impl<
        A: 'static,
        B: 'static,
        C: 'static,
        D: 'static,
        E: 'static,
        SA: IntoAccessor<Inner = A>,
        SB: IntoAccessor<Inner = B>,
        SC: IntoAccessor<Inner = C>,
        SD: IntoAccessor<Inner = D>,
        SE: IntoAccessor<Inner = E>,
    > ScopeItems for (SA, (SB, (SC, (SD, (SE, ())))))
{
    type Unpack = (
        Accessor<SE>,
        Accessor<SD>,
        Accessor<SC>,
        Accessor<SB>,
        Accessor<SA>,
    );

    fn resolve(items: &PolyMap) -> Self::Unpack {
        (
            infallible(items.get::<StoredItem<E>>().map(SE::accessor_for)),
            infallible(items.get::<StoredItem<D>>().map(SD::accessor_for)),
            infallible(items.get::<StoredItem<C>>().map(SC::accessor_for)),
            infallible(items.get::<StoredItem<B>>().map(SB::accessor_for)),
            infallible(items.get::<StoredItem<A>>().map(SA::accessor_for)),
        )
    }
}

fn infallible<T, E: Debug>(result: Result<T, E>) -> T {
    match result {
        Ok(value) => value,
        Err(msg) => {
            unreachable!("Type Error! Scope<..> {:?}", msg)
        }
    }
}

type Accessor<T> = <T as IntoAccessor>::Accessor;
