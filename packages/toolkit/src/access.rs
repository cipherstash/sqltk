use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

/// An enforced read-only wrapper around an [`Rc<RefCell<T>>`].
///
/// This allows visitors upstream in the pipeline to provide a read only view of
/// their data to downstream visitors.
///
/// It is not possible to obtain a reference to the underlying
/// [`Rc<RefCell<T>>`] as this would circumvent the mutability controls.
///
#[derive(Debug, Clone)]
pub struct ReadOnly<T> {
    value: Rc<RefCell<T>>,
}

impl<T> ReadOnly<T> {
    /// Create a new read-only view of the [`Rc<RefCell<T>>`].
    pub fn new(value: Rc<RefCell<T>>) -> Self {
        Self { value }
    }

    /// Immutably borrows the [`Rc<RefCell<T>>`] and returns a [`Ref<T>`].
    pub fn get(&self) -> Ref<T> {
        self.value.borrow()
    }
}

/// An read-write wrapper around an [`Rc<RefCell<T>>`].
///
/// It is not possible to obtain a reference to the underlying
/// [`Rc<RefCell<T>>`] as this would circumvent any mutability controls.
///
#[derive(Debug, Clone)]
pub struct ReadWrite<T> {
    value: Rc<RefCell<T>>,
}

impl<T> ReadWrite<T> {
    /// Create a new read-write view of the [`Rc<RefCell<T>>`].
    pub fn new(value: Rc<RefCell<T>>) -> Self {
        Self { value }
    }

    /// Immutably borrows the [`Rc<RefCell<T>>`] and returns a [`Ref<T>`].
    pub fn get(&self) -> Ref<T> {
        self.value.borrow()
    }

    /// Mutably borrows the [`Rc<RefCell<T>>`] and returns a [`RefMut<T>`].
    pub fn get_mut(&self) -> RefMut<T> {
        self.value.borrow_mut()
    }
}