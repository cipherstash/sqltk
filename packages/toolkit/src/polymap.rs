#![allow(dead_code)]

// IDEA: if this module turns out to be generally useful, publish it standalone.

use std::any::{type_name, TypeId};
use std::cell::RefCell;
use std::collections::{hash_map, HashMap};
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

/// A `PolyMap` is similar to a [`HashMap`] but it can store and retrieve keys
/// and values of different types simultaneously, whereas a regular
/// [`std::collections::HashMap`] can only store keys and values of fixed types.
///
/// Values are returned as [`Rc<RefCell<Value>>`].
///
/// # Limitations
///
/// `PolyMap` does not support deletion of entries.
///
/// It was built to support type-safe data sharing between
/// [`crate::UniversalVisitor`] implementations in a [`crate::Pipeline`]. The
/// pipelines are short-lived.
///
/// # Example
///
/// ```no_compile
/// use crate::polymap::{PolyMapKey, PolyMap};
/// use std::cell::RefCell;
/// use std::rc::Rc;
///
/// let mut polymap = PolyMap::new();
///
/// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// struct MyVecOfBool;
///
/// impl PolyMapKey for MyVecOfBool {
///     type PolyMapValue = Vec<bool>;
/// }
///
/// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// struct MyVecOfString;
///
/// impl PolyMapKey for MyVecOfString {
///     type PolyMapValue = Vec<String>;
/// }
///
/// let _ = polymap.insert::<MyVecOfBool>(
///     Rc::new(RefCell::new(vec![true, false])));
///
/// let _ = polymap.insert::<MyVecOfString>(
///     Rc::new(RefCell::new(vec!["Hello".into(), "World".into()])));
///
/// let vec_bool: Rc<RefCell<Vec<bool>>> =
///     polymap.get::<MyVecOfBool>().unwrap();
///
/// let vec_string: Rc<RefCell<Vec<String>>> =
///     polymap.get::<MyVecOfString>().unwrap();
/// ```
///
/// # Design notes
///
/// `PolyMap` employs a technique to simulate existential types in Rust (see
/// [`InternalValue`]).
///
/// Roughly speaking, an existential type `A` is a type that is parameterised by
/// another type `B` but without the `B` becoming part of the `A`'s signature.
/// Being able to "hide" the parameter means we can convince Rust that multiple
/// `A`s storing different `B`'s are all the same type: simply `A` instead of
/// `A<B>`.
///
/// This is achieved by wrapping a value like this: `Rc<RefCell<T>>`, then
/// converting the wrapped type to a `*const usize` pointer, thus hiding the
/// type `T`.
///
/// In order to recover the type `T` when getting values from the `PolyMap`,
/// just before the conversion to a pointer occurs a closure is created that
/// captures the type `T` and that closure is stored in a struct that also
/// contains the pointer.
///
/// The closure is executed with the pointer as an argument and the
/// `Rc<RefCell<T>>` is restored.
///
/// # Safety
///
/// The implementation makes use of `unsafe` Rust but upholds safety guarantees.
///
/// The unsafe function [`Rc::from_raw`] is used to convert a raw pointer back
/// into a `Rc<RefCell<T>>`.
///
pub struct PolyMap {
    config: RefCell<HashMap<InternalKey, InternalValue>>,
}

impl PolyMap {
    /// Creates a new empty `PolyMap`
    pub fn new() -> Self {
        Self {
            config: RefCell::new(HashMap::new()),
        }
    }
}

/// Trait for types used as keys in a `PolyMap`.
pub trait PolyMapKey: 'static + Debug {
    /// The type of the value retrieved by this key.
    type PolyMapValue: Debug;
}

impl PolyMap {
    /// Takes ownership of `value` and inserts a new entry into `Self`.
    ///
    /// Returns `Ok(())` on success.
    ///
    /// If an entry already exists it returns `Err(String)`.
    pub fn insert<Key: PolyMapKey>(&mut self, value: Key::PolyMapValue) -> Result<(), String> {
        self.insert_internal::<Key>(Rc::new(RefCell::new(value)))
            .map_or(Some(()), |_| None)
            .ok_or(format!(
                "PolyMap already contains a config key of type {}",
                type_name::<Key>()
            ))
    }

    /// Gets a value using the type `Key` as a hash key.
    ///
    /// Returns `Ok(Rc<RefCell<Key::PolyMapValue>>)` on success.
    ///
    /// If a value for type `Key` does not exist, `Err(String)` is returned.
    pub fn get<Key: PolyMapKey>(&self) -> Result<Rc<RefCell<Key::PolyMapValue>>, String> {
        self.get_internal::<Key>().ok_or(format!(
            "PolyMap does not contain key of type {}",
            type_name::<Key>(),
        ))
    }

    /// Checks if there is a stored value for type `Key`.
    pub fn contains_key<Key: PolyMapKey>(&self) -> bool {
        let key = InternalKey {
            type_id: TypeId::of::<Key>(),
            type_name: type_name::<Key>(),
            clone_to_other: Rc::new(RefCell::new(Box::new(
                |_from: &PolyMap, _to: &mut PolyMap| {},
            ))),
        };

        self.config.borrow().contains_key(&key)
    }

    /// Returns the number of entries in this `PolyMap`.
    pub fn len(&self) -> usize {
        self.config.borrow().len()
    }

    fn get_internal<Key: PolyMapKey>(&self) -> Option<Rc<RefCell<Key::PolyMapValue>>> {
        let key = InternalKey {
            type_id: TypeId::of::<Key>(),
            type_name: type_name::<Key>(),
            clone_to_other: Rc::new(RefCell::new(Box::new(
                |_: &PolyMap, _: &mut PolyMap| {},
            ))),
        };

        let config_hash = self.config.borrow();
        config_hash
            .get(&key)
            .map(|value| unsafe { value.get::<Key::PolyMapValue>() })
    }

    fn insert_internal<Key: PolyMapKey>(
        &mut self,
        value: Rc<RefCell<Key::PolyMapValue>>,
    ) -> Option<()> {
        let key = InternalKey {
            type_id: TypeId::of::<Key>(),
            type_name: type_name::<Key>(),
            clone_to_other: Rc::new(RefCell::new(Box::new(
                |from: &PolyMap, to: &mut PolyMap| {
                    let _ = from
                        .get_internal::<Key>()
                        .map(|value| to.insert_internal::<Key>(value))
                        .expect("Unreachable: failed to clone value");
                },
            ))),
        };

        let config_hash = self.config.get_mut();

        config_hash
            .insert(key, InternalValue::new(value))
            .map(|_| ())
    }
}

impl Clone for PolyMap {
    fn clone(&self) -> Self {
        // The closure limits the scope of the borrow of self.config.
        // self.config is re-borrowed inside the `clone_to_other` closure so
        // limiting the scope of the borrow is vital.
        let clone_keys = || -> Vec<InternalKey> {
            let mut cloned_keys: Vec<InternalKey> = Vec::new();
            let config = self.config.borrow();
            for key in config.keys() {
                cloned_keys.push(key.clone());
            }
            cloned_keys
        };

        let mut new_polymap = PolyMap::new();

        for key in clone_keys() {
            // Key has access to concrete type of value, it can safely
            // reconstruct an `Rc` for the value and put it into the new PolyMap.
            (*key.clone_to_other.borrow())(&self, &mut new_polymap);
        }

        new_polymap
    }
}

impl IntoIterator for PolyMap {
    type Item = (InternalKey, InternalValue);

    type IntoIter = hash_map::IntoIter<InternalKey, InternalValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.config.into_inner().into_iter()
    }
}

impl Extend<(InternalKey, InternalValue)> for PolyMap {
    fn extend<T: IntoIterator<Item = (InternalKey, InternalValue)>>(&mut self, iter: T) {
        let config = self.config.get_mut();
        for (k, v) in iter {
            config.insert(k, v);
        }
    }
}

/// Used as the actual key type in the underlying `HashMap`.
#[derive(Clone)]
#[doc(hidden)]
pub struct InternalKey {
    /// `TypeId` of the `PolyMapKey` implementation. This is the only field used
    /// in the [`Hash`] implementation.
    type_id: TypeId,
    /// Human readable type name. Used in error messages and for debugging
    /// purposes only.
    type_name: &'static str,
    /// Closure that knows the concrete key and value types and can safely clone
    /// this key from one `PolyMap` into another `PolyMap`.
    clone_to_other: Rc<RefCell<Box<dyn Fn(&PolyMap, &mut PolyMap)>>>,
}

impl Debug for InternalKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Key")
            .field("type_name", &self.type_name)
            .finish()
    }
}

impl PartialEq for InternalKey {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id
    }
}

impl Eq for InternalKey {}

impl Hash for InternalKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.type_id.hash(state);
    }
}

/// A type that can hold an Rc<RefCell<T>> for any T but "hides" type T such
/// that it is not part of the type signature of Val.
///
/// This can be thought of as simulating runtime existential types, and allows
/// us to build a type-heterogenous HashMap where values can vary in type.
///
/// # Safety
///
/// This type makes use of unsafe code. All public functions are guaranteed to
/// uphold type safety rules and will not result in undefined behaviour.
#[doc(hidden)]
pub struct InternalValue {
    /// A pointer to a [`RefCell`] of some type.
    ptr: usize,
    /// Closure that is called when the `InternalValue` is dropped in order to
    /// decrement the strong count of the `Rc` that was turned into a raw
    /// pointer.
    decrement_strong_count: Box<dyn Fn()>,
}

impl InternalValue {
    /// Creates a new `Val` that retains a reference to an [`Rc<RefCell<T>>`],
    /// while not requiring `Val` have type `T` as part of its type signature.
    ///
    /// # Safety
    ///
    /// Internally this type makes use of `unsafe` (specifically
    /// [`Rc::into_raw`] and [`Rc::from_raw`] in order to hold onto a value
    /// while eliding its type.  But `InternalValue` upholds the safety
    /// guarantees by ensuring that the strong reference count of the `Rc` is
    /// decremented when the `InternalValue` is dropped.
    fn new<V: Debug>(value: Rc<RefCell<V>>) -> Self {
        // Capture the memory address to the content of the Rc and save it.
        // Turning an `Rc` into an address means that the type parameter `V`
        // can be discarded.
        let ptr = Rc::into_raw(value) as usize;

        // To avoid a memory leak when Self is dropped, the strong count must be
        // decremented and this is only safe when being provided a `*const
        // RefCell<V>`. The closure captures its environment which includes the
        // type V and we can store this closure call it in the `Drop`
        // implementation.
        Self {
            ptr,
            decrement_strong_count: Self::decr_strong_count_on_drop::<V>(ptr),
        }
    }

    fn decr_strong_count_on_drop<V>(ptr: usize) -> Box<dyn Fn()> {
        Box::new(move || {
            unsafe { Rc::decrement_strong_count(ptr as *const RefCell<V>) }
        })
    }

    /// Gets the Rc<RefCell<V>> out of the [`InternalVal`].
    ///
    /// # Safety
    ///
    /// This function is unsafe because calling it with the wrong `V` is
    /// undefined behaviour. However this function is private and is only
    /// invoked when the type for `V` is know to be correct.
    unsafe fn get<V: Debug>(&self) -> Rc<RefCell<V>> {
        let ptr = self.ptr as *const RefCell<V>;
        // The `Rc` returned from `from_raw` will go out of scope at the end of
        // this function, but the `PolyMap` must retain a strong reference as it
        // will be left holding only a usize pointer to its memory address. So
        // increment the strong count.
        Rc::increment_strong_count(ptr);
        // Recover the `Rc` using size and alignment of `V` .
        Rc::from_raw(ptr)
    }
}

impl Drop for InternalValue {
    fn drop(&mut self) {
        (*self.decrement_strong_count)()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct FooKey;

    #[derive(Debug)]
    struct FooValue;

    impl PolyMapKey for FooKey {
        type PolyMapValue = FooValue;
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct BarKey;

    #[derive(Debug)]
    struct BarValue;

    impl PolyMapKey for BarKey {
        type PolyMapValue = BarValue;
    }

    #[test]
    fn insert_and_get() {
        let mut polymap = PolyMap::new();

        match polymap.insert::<FooKey>(FooValue) {
            Ok(()) => match polymap.get::<FooKey>() {
                Ok(_value) => assert!(true),
                Err(msg) => assert!(false, "{msg}"),
            },
            Err(msg) => assert!(false, "{msg}"),
        }

        match polymap.insert::<BarKey>(BarValue) {
            Ok(()) => match polymap.get::<BarKey>() {
                Ok(_value) => assert!(true),
                Err(msg) => assert!(false, "{msg}"),
            },
            Err(msg) => assert!(false, "{msg}"),
        }
    }

    #[test]
    fn clone() {
        let mut polymap = PolyMap::new();

        polymap.insert::<FooKey>(FooValue).unwrap();
        polymap.insert::<BarKey>(BarValue).unwrap();

        let polymap = polymap;

        let cloned = polymap.clone();

        match cloned.get::<FooKey>() {
            Ok(_value) => assert!(true),
            Err(msg) => assert!(false, "{msg}"),
        }

        match cloned.get::<BarKey>() {
            Ok(_value) => assert!(true),
            Err(msg) => assert!(false, "{msg}"),
        }

        // check the source PolyMap still works as expected
        match polymap.get::<FooKey>() {
            Ok(_value) => assert!(true),
            Err(msg) => assert!(false, "{msg}"),
        }

        match polymap.get::<BarKey>() {
            Ok(_value) => assert!(true),
            Err(msg) => assert!(false, "{msg}"),
        }
    }

    #[test]
    fn extend() {
        let mut polymap = PolyMap::new();

        polymap.insert::<FooKey>(FooValue).unwrap();
        polymap.insert::<BarKey>(BarValue).unwrap();

        let mut new_polymap = PolyMap::new();

        new_polymap.extend(polymap);

        assert_eq!(new_polymap.len(), 2);
    }

    #[test]
    fn drop_works() {
        let mut polymap = PolyMap::new();

        polymap.insert::<FooKey>(FooValue).unwrap();

        let foo = polymap.get::<FooKey>().unwrap();

        // Holding a weak ref lets us test strong_count going to zero.
        let foo_weak = Rc::downgrade(&foo);
        drop(foo);

        assert_eq!(foo_weak.strong_count(), 1);

        let retrieved_foo = polymap.get::<FooKey>().unwrap();

        assert_eq!(foo_weak.strong_count(), 2);

        let cloned_polymap = polymap.clone();

        assert_eq!(foo_weak.strong_count(), 3);

        drop(polymap);

        assert_eq!(foo_weak.strong_count(), 2);

        drop(cloned_polymap);

        assert_eq!(foo_weak.strong_count(), 1);

        drop(retrieved_foo);

        assert_eq!(foo_weak.strong_count(), 0);
    }

    #[test]
    fn mutation() {
        let mut polymap = PolyMap::new();

        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
        struct SomeString;

        impl PolyMapKey for SomeString {
            type PolyMapValue = String;
        }

        polymap
            .insert::<SomeString>(String::from("Hello!"))
            .unwrap();

        let value: Rc<RefCell<String>> = polymap.get::<SomeString>().unwrap();

        {
            let string = value.borrow();
            assert_eq!("Hello!", *string);
        }

        {
            let mut string = value.borrow_mut();
            string.make_ascii_uppercase();
            assert_eq!("HELLO!", *string);
        }

        let value: Rc<RefCell<String>> = polymap.get::<SomeString>().unwrap();

        let string = value.borrow();
        assert_eq!("HELLO!", *string);
    }

    #[test]
    fn mutation_2() {
        let mut polymap = PolyMap::new();

        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
        struct SomeU64;

        impl PolyMapKey for SomeU64 {
            type PolyMapValue = u64;
        }

        polymap.insert::<SomeU64>(123456789).unwrap();

        let value1: Rc<RefCell<u64>> = polymap.get::<SomeU64>().unwrap();

        {
            let number = value1.borrow();
            assert_eq!(123456789, *number);
        }

        {
            let mut number = value1.borrow_mut();
            *number = *number * 2;
            assert_eq!(246913578, *number);
        }

        let value2: Rc<RefCell<u64>> = polymap.get::<SomeU64>().unwrap();

        assert!(Rc::ptr_eq(&value1, &value2));

        let number = value2.borrow();
        assert_eq!(246913578, *number);
    }
}
