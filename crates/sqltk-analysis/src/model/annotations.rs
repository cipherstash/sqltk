//! Associate arbitrary information with AST nodes

use core::{
    ops::{Deref, DerefMut},
    {any::TypeId, fmt::Debug, marker::PhantomData},
};
use std::collections::HashMap;
use std::rc::Rc;

use sqlparser::ast::{Query, Select, SetExpr};
use sqltk::prelude::{Expr, SelectItem, Visitable};

use crate::{projection_annotation::ProjectionAnnotation, source_annotation::SourceAnnotation};

/// Key-value store that can associate a specific type of value with any
/// type of key where the key type has a `'static` lifetime.
///
/// The intent is to be able to associate derived semantic information about
/// `sqlparser` AST nodes.
///
/// `sqlparser` does not give AST nodes _identity_, which means we have to
/// resort to using the memory address of the node. This may _seem_ fragile but
/// the [`AnnotationKey`] captures the lifetime of the node which prevents the
/// node from being moved - which would invalidate its address for further
/// lookups.
///
/// Memory addresses of nodes cannot be relied upon to be unique (e.g. base
/// address of a struct will be shared with the address of its first field),
/// so in addition to the address, the [`TypeId`] of the node is used.
///
/// `AnnotationStore` implements [`Deref`] & [`DerefMut`] with [`HashMap`]
/// as the `Target` so it can be used as a regular `HashMap`.
///
/// There is a blanket implementation for [From] for [AnnotationKey] for all
/// [`Visitable`](sqltk::Visitable).
///
/// # Example
///
/// ```
/// # use sqltk_analysis::annotations::{AnnotationKey, AnnotationStore};
/// # use std::collections::HashMap;
///
/// let mut store = AnnotationStore::<u8>::default();
/// let s1 = String::from("HELLO");
/// let s2 = String::from("GOODBYE");
/// let b1 = true;
/// let b2 = false;
///
/// store.insert((&s1).into(), 10);
/// store.insert((&s2).into(), 11);
/// store.insert((&b1).into(), 20);
/// store.insert((&b2).into(), 21);
///
/// assert_eq!(Some(&10), store.get(&(&s1).into()));
/// assert_eq!(Some(&11), store.get(&(&s2).into()));
/// assert_eq!(Some(&20), store.get(&(&b1).into()));
/// assert_eq!(Some(&21), store.get(&(&b2).into()));
/// ```
///
/// # Failing example due to attempt to drop node
///
/// ```compile_fail
/// # use sqltk_analysis::annotations::{AnnotationKey, AnnotationStore};
/// # use std::collections::HashMap;
/// let mut store = AnnotationStore::<u8>::default();
/// let some_string = String::from("HELLO");
///
/// let key: AnnotationKey = (&some_string).into();
/// store.insert(key.clone(), 100);
///
/// assert_eq!(Some(&100), store.get(&key));
/// drop(some_string);
/// assert_eq!(Some(&100), store.get(&key));
/// ```
/// ## Error:
///
/// ```text
/// error[E0505]: cannot move out of `some_string` because it is borrowed
/// ```
#[derive(Debug)]
pub struct AnnotationStore<'key, V>(HashMap<AnnotationKey<'key>, V>);

impl<'key, V> Default for AnnotationStore<'key, V> {
    fn default() -> Self {
        Self(HashMap::default())
    }
}

impl<'key, V> AnnotationStore<'key, V> {
    /// Returns an [`Iterator`] of all stored [`AnnotationKey`]s where the
    /// [`TypeId`] stored in the `AnnotationKey` is equal to
    /// `TypeId::of::<K>()`.
    ///
    /// For example, `keys_of_type<Expr>()` will return an `Iterator<Item =
    /// &AnnotationKey<'key>>` for all annotations with a key derived from an
    /// `Expr` node.
    pub fn keys_of_type<K: 'static>(&self) -> impl Iterator<Item = &AnnotationKey<'key>> {
        self.keys()
            .filter(|AnnotationKey(type_id, _, _)| TypeId::of::<K>() == *type_id)
    }

    /// Returns an [`Iterator`] of all stored values where the
    /// [`TypeId`] stored in the [`AnnotationKey`] is equal to
    /// `TypeId::of::<K>()`.
    ///
    /// For example, `values_for_key_type<Expr>()` will return an `Iterator<Item
    /// = &V>` for all annotations with a key derived from an `Expr` node.
    pub fn values_for_key_type<K: 'static>(&self) -> impl Iterator<Item = &V> {
        self.keys_of_type::<K>().map(|key| self.get(key).unwrap())
    }
}

/// The key type used for insertion and retrieval of items in an [`AnnotationStore`].
///
/// Internally the key is composed of a [`TypeId`] and a memory address
/// (represented as a `usize`).
///
/// See the documentation for [`AnnotationStore`] for more information about why
/// memory address alone is insufficient.
///
/// SAFETY: the `usize` memory address is never dereferenced. There is no unsafe
/// code in `AnnotationStore.`
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct AnnotationKey<'key>(TypeId, usize, PhantomData<&'key ()>);

impl<'ast, T: 'static> From<&'ast T> for AnnotationKey<'ast> {
    fn from(node: &'ast T) -> Self {
        Self(TypeId::of::<T>(), node as *const T as usize, PhantomData)
    }
}

// AnnotationStore behaves like a HashMap<AnnotationKey<'key>, V>.
impl<'key, V> Deref for AnnotationStore<'key, V> {
    type Target = HashMap<AnnotationKey<'key>, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'key, V> DerefMut for AnnotationStore<'key, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Trait that is implemented by [`AnnotationStore`] for every AST node type
/// that it supports.
///
/// The reason this trait exists is to make it a compilation error to try to
/// read an annotation from an AST node type for which analysis has not yet been
/// implemented.
pub trait Annotates<'ast, N: Visitable<'ast>, A> {
    /// Adds an annotation of type `A` for an AST node `self`.
    ///
    /// Panics if an annotation of type `A` is already present for `node`.
    fn add_annotation(&mut self, node: &'ast N, annotation: impl Into<Rc<A>>) -> Rc<A>;

    /// Gets an annotation of type `Rc<A>` from `self` for a particular AST node
    /// `node`. Returns `Some<Rc<A>>` if an annotation is present on the AST node
    /// or `None` if it does not exist.
    fn get_annotation(&self, node: &'ast N) -> Option<Rc<A>>;

    /// Same as [`Annotates::get_annotation`], but returns a [`Result`] instead
    /// of an [`Option`].
    ///
    /// Use this method when specific annotations are expected to be present.
    fn expect_annotation(&self, node: &'ast N) -> Result<Rc<A>, ExpectedAnnotationError<A>>;
}

/// Error returned when attempting to retrieve an expected annotation when it is not present.
#[derive(Debug, thiserror::Error)]
#[error(
    "Expected {} annotation on AST node: {}",
    "std::any::type_name::<A>()",
    _0
)]
pub struct ExpectedAnnotationError<A>(pub String, PhantomData<A>);

macro_rules! annotates {
    ($node:ty, $annotation:ty) => {
        impl<'ast> Annotates<'ast, $node, $annotation> for AnnotationStore<'ast, Rc<$annotation>> {
            fn add_annotation(
                &mut self,
                node: &'ast $node,
                annotation: impl Into<Rc<$annotation>>,
            ) -> Rc<$annotation> {
                let key = AnnotationKey::from(node);
                let annotation: Rc<$annotation> = annotation.into();

                if let Some(_) = self.insert(key, annotation) {
                    panic!(
                        "Already an existing {} on node",
                        std::any::type_name::<$annotation>()
                    );
                }

                self.get(&key)
                    .expect("to get the entry that was just added")
                    .clone()
            }

            fn get_annotation(&self, node: &'ast $node) -> Option<Rc<$annotation>> {
                self.get(&AnnotationKey::from(node)).cloned()
            }

            fn expect_annotation(
                &self,
                node: &'ast $node,
            ) -> Result<Rc<$annotation>, ExpectedAnnotationError<$annotation>> {
                self.get_annotation(node)
                    .ok_or(ExpectedAnnotationError(node.to_string(), PhantomData))
            }
        }
    };
}

// TODO: move these, they don't belong here
annotates!(Expr, SourceAnnotation);
annotates!(SelectItem, SourceAnnotation);
annotates!(Expr, ProjectionAnnotation);
annotates!(Query, ProjectionAnnotation);
annotates!(Select, ProjectionAnnotation);
annotates!(SetExpr, ProjectionAnnotation);
