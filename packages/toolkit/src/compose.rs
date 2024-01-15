// use std::{marker::PhantomData};

// use crate::UniversalVisitor;

// /// A cons-cell type for representing type-level and value-level a singly linked
// /// list.
// ///
// /// At compile time it represents a list of types which is used for defining
// /// producer-consumer contracts between visitor implementations in a pipeline.
// ///
// /// At runtime it contains values of those types.
// ///
// /// `Head` can be any type. `Tail` must implement [`TList`]. [`Cons`] itself
// /// implements [`TList`].
// ///
// /// [`TList`] expects a chain of `Cons` cells to be terminated by [`()`].
// pub struct Cons<Head, Tail>(Head, Tail);

// /// A trait for abstracting over linked lists of types and/or values.
// pub trait TList {
//     type Head;
//     type Tail: TList;
// }

// impl TList for () {
//     type Head = ();
//     type Tail = ();
// }

// impl<H, T: TList> TList for Cons<H, T> {
//     type Head = H;
//     type Tail = T;
// }

// pub trait Has<T> {
//     type Output: Bool;
// }

// /// NOTE: when use of TypeId::of() in const contexts is stabilised we'll be able
// /// to provide a blanket implementation of IsSameType. Custom auto-traits and
// /// trait specialisation are also potential solutions to providing a blanket
// /// implementation.
// //
// /// // Solution 1: Using TypeId::of() in const context:
// //
// /// impl<A: 'static, B: 'static> SameType for (A, B) {
// ///     const Output: bool = TypeId::of::<A>() == TypeId::of::<B>();
// /// }
// //
// /// Solution 2: using auto traits and negative trait bounds:
// //
// /// // Every type gets this auto trait.
// /// auto trait DefaultIsSameType {};
// //
// /// // For every T: DefaultSameType create a SameType impl that default to false
// /// impl<T: DefaultIsSameType> SameType for T {
// ///     const Output: bool = false;
// /// }
// //
// /// // Opt-out of the autotrait for every 2-tuple where each type in the tuple is
// /// // the same type.
// /// impl<T> !DefaultIsSameType for (T, T)
// //
// /// // Implement IsSameType for every 2-tuple where each type is the same type.
// /// // The autotrait will still cover every type where the tuple types are
// /// // different.
// //
// /// impl<T> IsSameType for (T, T) {
// ///     const Output: bool = true;
// /// }
// ///
// pub trait IsSameType {
//     type Output: Bool;
// }

// /// Type-level `bool`
// pub trait Bool {
//     const VALUE: bool;
// }

// /// Type-level `true`
// pub struct True;
// impl Bool for True {
//     const VALUE: bool = true;
// }

// /// Type-level `false`
// pub struct False;
// impl Bool for False{
//     const VALUE: bool = false;
// }

// pub trait ToBool {
//     type Output: Bool;
// }

// pub struct ConvertBool<const VALUE: bool>();

// impl ToBool for ConvertBool<true> {
//     type Output = True;
// }

// impl ToBool for ConvertBool<false> {
//     type Output = False;
// }

// impl<A, B, Tail: TList + Has<A>> Has<A> for Cons<B, Tail>
// where
//     (A, B): IsSameType,
// {
//     type Output =
//         <
//             Or<
//                 <(A, B) as IsSameType>::Output,
//                 <Tail as Has<A>>::Output
//             >
//             as Eval
//         >::Output;

//     // type Output = Or<
//     //             <(A, B) as IsSameType>::Output,
//     //             <Tail as Has<A>>::Output
//     //         >;
// }

// pub struct Or<A: Bool, B: Bool>(PhantomData<(A, B)>);

// trait Eval {
//     type Output: Bool;
// }

// impl<A: Bool, B: Bool> Eval for Or<A, B> {
//    type Output = <ConvertBool<{A::VALUE || B::VALUE}> as ToBool>::Output;
// }

// impl Eval for Or<False, False> {
//    type Output = False;
// }

// impl Eval for Or<True, False> {
//    type Output = True;
// }

// impl Eval for Or<False, True> {
//    type Output = True;
// }

// impl Eval for Or<True, True> {
//    type Output = True;
// }

// pub struct And<A, B>(PhantomData<(A, B)>);

// impl Eval for And<False, False> {
//    type Output = False;
// }

// impl Eval for And<True, False> {
//    type Output = False;
// }

// impl Eval for And<False, True> {
//    type Output = False;
// }

// impl Eval for And<True, True> {
//    type Output = True;
// }

// impl<A> Has<A> for () {
//     type Output = False;
// }

// pub trait HasAll<Required: TList> {
//     type Output;
// }

// impl<H, T: TList> HasAll<Cons<H, T>> for () {
//     type Output = False;
// }

// impl HasAll<()> for () {
//     type Output = True;
// }

// impl<
//         Required: TList,
//         ProvidedHead,
//         ProvidedTail: TList + Has<Required::Head> + HasAll<Required::Tail>,
//     > HasAll<Required> for Cons<ProvidedHead, ProvidedTail>
// where
//     (<Required as TList>::Head, ProvidedHead): IsSameType,
// {
//     type Output =
//         <And<<Self as Has<Required::Head>>::Output, <ProvidedTail as HasAll<Required::Tail>>::Output> as Eval>::Output;
// }

// pub enum PipelineError {}

// pub trait Init where Self: UniversalVisitor {
//     type Requires: TList;
//     type Publishes: TList;

//     fn init(args: &Self::Requires) -> (Self, Self::Publishes);
// }

// pub struct PipelineStage<Provides: TList, Visitors: TList> {
//     visitors: Visitors,
//     provides: Provides,
// }

// impl<StagePublishes: TList, Visitors: TList> PipelineStage<StagePublishes, Visitors> {
//     pub fn connect<Initializer: Init>(
//         &self,
//     ) -> Result<
//         PipelineStage<
//             Cons<Initializer::Publishes, StagePublishes>,
//             Cons<Initializer, Visitors>,
//         >,
//         PipelineError,
//     > {
//         // 1. build an Initializer::Requires from StagePublishes
//         // 2. Initiize the visitor
//         // 3. Return the new pipeline stage
//         todo!()
//     }
// }

// pub fn new_pipeline<Initializer: Init<Requires = ()>>() -> PipelineStage<
//     Initializer::Publishes,
//     Cons<Initializer, ()>,
// > {
//     let (visitor, provides) = Initializer::init(&());
//     PipelineStage {
//         visitors: Cons(visitor, ()),
//         provides,
//     }
// }
