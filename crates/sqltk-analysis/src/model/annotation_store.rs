//! Associate arbitrary information with AST nodes

use core::{fmt::Debug, marker::PhantomData};
use std::any::TypeId;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

/// Associates arbitrary metadata with AST nodes.
#[derive(Debug)]
pub struct AnnotationStore<'ast, N: 'static, V> {
    storage: HashMap<NodeKey<'ast, N>, Rc<V>>,
}

/// `NodeKey` ensures every node in the AST has a unique identity.
///
/// AST nodes from `sqlparser` do not have a unique identifier, which implies
/// that two nodes of the same type and value but different parents would
/// compare as equal with `PartialEq`.
///
/// The [`Hash`] & [`PartialEq`] implementations for `NodeKey` take into account
/// the [`TypeId`] and memory address of the node to ensure uniqueness.
///
/// Note: taking into account the type is necessary because multiple nodes can
/// have the same address, e.g. the address of a struct and the address of its
/// first field will be the same.
#[derive(Debug, Clone)]
struct NodeKey<'ast, N: 'static>(&'ast N);

impl<'ast, N: 'static> Hash for NodeKey<'ast, N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let ty = TypeId::of::<N>();
        let addr = self.0 as *const N as usize;
        ty.hash(state);
        addr.hash(state);
    }
}

impl<'ast, N: 'static> PartialEq for NodeKey<'ast, N> {
    fn eq(&self, other: &Self) -> bool {
        let self_addr = self.0 as *const N as usize;
        let other_addr = other.0 as *const N as usize;
        self_addr == other_addr
    }
}

impl<'ast, N: 'static> Eq for NodeKey<'ast, N> {}

impl<'ast, N, V> AnnotationStore<'ast, N, V> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'ast, N, V> Default for AnnotationStore<'ast, N, V> {
    fn default() -> Self {
        Self {
            storage: HashMap::default(),
        }
    }
}

pub trait Annotates<'ast, N, A> {
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

impl<'ast, N, A> Annotates<'ast, N, A> for AnnotationStore<'ast, N, A>
where
    A: Debug,
    N: 'static + Clone + Debug + Display,
{
    fn add_annotation(&mut self, node: &'ast N, annotation: impl Into<Rc<A>>) -> Rc<A> {
        let annotation: Rc<A> = annotation.into();
        let key = NodeKey(node);

        if let Some(existing) = self.storage.insert(key.clone(), annotation) {
            panic!("Already an existing {:#?} on node {:#?}", existing, node)
        }

        self.storage
            .get(&key)
            .expect("to get the entry that was just added")
            .clone()
    }

    fn get_annotation(&self, node: &'ast N) -> Option<Rc<A>> {
        let key = NodeKey(node);
        self.storage.get(&key).cloned()
    }

    fn expect_annotation(&self, node: &'ast N) -> Result<Rc<A>, ExpectedAnnotationError<A>> {
        self.get_annotation(node)
            .ok_or(ExpectedAnnotationError(node.to_string(), PhantomData))
    }
}

/// Error returned when attempting to retrieve an expected annotation when it is
/// not present.
#[derive(Debug, thiserror::Error, Eq, PartialEq, PartialOrd, Ord)]
#[error(
    "Expected {} annotation on AST node: {}",
    "std::any::type_name::<A>()",
    _0
)]
pub struct ExpectedAnnotationError<A>(pub String, PhantomData<A>);
