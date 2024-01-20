use std::{cell::RefCell, rc::Rc};

use super::accessors::{ReadOnly, ReadWrite};
use super::{ExportedItem, ImportedItem, StoredItem};

pub trait IntoAccessor {
    type Accessor;
    type Inner;

    fn accessor_for(value: Rc<RefCell<Self::Inner>>) -> Self::Accessor;
}

impl<T> IntoAccessor for ExportedItem<T> {
    type Accessor = ReadWrite<T>;
    type Inner = T;

    fn accessor_for(value: Rc<RefCell<Self::Inner>>) -> Self::Accessor {
        ReadWrite::new(value)
    }
}

impl<T> IntoAccessor for ImportedItem<T> {
    type Accessor = ReadOnly<T>;
    type Inner = T;

    fn accessor_for(value: Rc<RefCell<Self::Inner>>) -> Self::Accessor {
        ReadOnly::new(value)
    }
}

impl<T> IntoAccessor for StoredItem<T> {
    type Accessor = ReadOnly<T>;
    type Inner = T;

    fn accessor_for(value: Rc<RefCell<Self::Inner>>) -> Self::Accessor {
        ReadOnly::new(value)
    }
}
