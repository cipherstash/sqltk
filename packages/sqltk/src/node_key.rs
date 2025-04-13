use std::{any::TypeId, marker::PhantomData};

/// Acts as a type-erased proxy for any type bound by `'static`.
///
/// The primary purpose is to be used as a key in a [`std::collections::HashMap`] where the key of the map is derived
/// from any type of AST node, effectively making the hashmap heterogeneous over the key type.
///
/// Additionally, the [`NodeKey::get_as`] method can *safely* get the value of the specific proxied type.
///
/// `NodeKey` works by capturing the address of an AST node in addition to its [`TypeId`]. Both are required to uniquely
/// identify a node because different node values can have the same address; for example the address of a struct and the
/// address of its first field will be equal but the struct and its first field are different types.
///
/// A `NodeKey` can only be created by [`AsNodeKey`] impls.
#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
pub struct NodeKey<'ast> {
    node_addr: usize,
    node_type: TypeId,
    _ast: PhantomData<&'ast ()>,
}

pub trait AsNodeKey
where
    Self: 'static,
{
    fn as_node_key(&self) -> NodeKey<'_>;
}

impl<N: AsNodeKey> AsNodeKey for Box<N> {
    fn as_node_key(&self) -> NodeKey<'_> {
        (**self).as_node_key()
    }
}

impl<N: 'static> AsNodeKey for Option<N> {
    fn as_node_key(&self) -> NodeKey<'_> {
        NodeKey::new(self)
    }
}

impl<N> AsNodeKey for Vec<N>
where
    N: AsNodeKey,
{
    fn as_node_key(&self) -> NodeKey<'_> {
        NodeKey::new(self)
    }
}

impl<'ast> NodeKey<'ast> {
    pub fn new<N: 'static>(node: &'ast N) -> Self {
        Self {
            node_addr: node as *const N as usize,
            node_type: TypeId::of::<N>(),
            _ast: PhantomData,
        }
    }

    pub fn get_as<N: 'static>(&self) -> Option<&'ast N> {
        if self.node_type == TypeId::of::<N>() {
            // SAFETY: we have verified that `N` is of the correct type to permit the cast and because `'ast` outlives
            // `self` we know that the node has not been dropped.
            unsafe { (self.node_addr as *const N).as_ref() }
        } else {
            None
        }
    }
}
