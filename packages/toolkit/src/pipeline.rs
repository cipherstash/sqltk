// use std::{rc::Rc, cell::RefCell};
// use std::fmt::Debug;

// use crate::ToUniversalVisitor;
// use crate::{polymap::{PolyMap, PolyMapKey}, ReadWrite, ReadOnly};

// pub trait Stage<'ast>: ToUniversalVisitor<'ast> {
//     fn init(data: PipelineData) -> Result<PipelineData, String>;
// }

// pub struct Pipeline<'ast> {
// }

// impl<'ast> Pipeline<'ast> {
//     pub fn new<T: Stages>(data: PipelineData) -> Self {
//     }
// }

// macro_rules! impl_stage {
//     ($ty:ty) => {

//     };
// }

// pub struct PipelineData {
//     storage: PolyMap,
// }

// impl PipelineData {
//     pub fn new() -> Self {
//         Self {
//             storage: PolyMap::new(),
//         }
//     }

//     pub fn insert<Key: DataKey>(&mut self, value: Key::Value) -> Result<(), String> {
//         self.storage.insert::<Key>(value)
//     }

//     pub fn get<Key: DataKey>(&mut self) -> Result<Key::Access, String> {
//         self.storage.get::<Key>().map(|value| Key::wrap(value))
//     }

//     pub fn len(&self) -> usize {
//         self.storage.len()
//     }
// }

// mod private {
//     pub trait Sealed {}
// }

// pub trait DataKey: 'static + private::Sealed + Debug {
//     type Value: Debug;
//     type Access: Debug;

//     fn wrap(value: Rc<RefCell<Self::Value>>) -> Self::Access;
// }

// impl<T> private::Sealed for ReadOnly<T> {}

// impl<T: 'static + Debug> DataKey for ReadOnly<T> {
//     type Value = T;
//     type Access = ReadOnly<Self::Value>;

//     fn wrap(value: Rc<RefCell<Self::Value>>) -> Self::Access {
//        ReadOnly::new(value)
//     }
// }

// impl<T> private::Sealed for ReadWrite<T> {}

// impl<T: 'static + Debug> DataKey for ReadWrite<T> {
//     type Value = T;
//     type Access = ReadWrite<Self::Value>;

//     fn wrap(value: Rc<RefCell<Self::Value>>) -> Self::Access {
//        ReadWrite::new(value)
//     }
// }

// impl<T: Debug + 'static> PolyMapKey for T where T: DataKey {
//     type PolyMapValue = T::Value;
// }

// #[cfg(test)]
// mod test {
//     use crate::ReadWrite;

//     use super::PipelineData;

//     #[test]
//     fn basic() {
//         let mut data = PipelineData::new();

//         data.insert::<ReadWrite<String>>("Hello".to_owned()).unwrap();

//         let retrieved = data.get::<ReadWrite<String>>().unwrap();
//         assert_eq!(*retrieved.get(), "Hello");

//         retrieved.get_mut().make_ascii_uppercase();
//         drop(retrieved);

//         let retrieved_again = data.get::<ReadWrite<String>>().unwrap();

//         assert_eq!(*retrieved_again.get(), String::from("HELLO"));
//     }

//     #[test]
//     fn compose() {

//     }
// }