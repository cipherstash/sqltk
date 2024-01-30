mod accessors;
mod into_accessor;
mod polymap;
mod scope_items;

use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub use accessors::*;
pub use into_accessor::IntoAccessor;

use polymap::{PolyMap, PolyMapKey};

use self::scope_items::ScopeItems;

#[derive(Default, Clone)]
pub struct RootScope {
    payload: Rc<RefCell<ScopePayload>>,
}

impl<'scope> RootScope {
    pub fn new() -> impl Scope<'scope, ()> {
        RootScope::default()
    }

    pub fn get<T: 'static>(&self) -> Result<ReadOnly<T>, UnknownItemError> {
        self.payload
            .borrow()
            .items
            .get::<StoredItem<T>>()
            .map(|value| ReadOnly::new(value))
            .map_err(UnknownItemError)
    }
}

#[derive(Default)]
struct ScopePayload {
    items: PolyMap,
    error: Option<UnknownItemError>,
}

pub trait Scope<'scope, Items: ScopeItems = ()>
where
    Self: 'scope + Clone,
{
    fn export<T: 'static>(&mut self, value: T) -> impl Scope<'scope, (ExportedItem<T>, Items)>
    where
        (ExportedItem<T>, Items): ScopeItems;

    fn import<T: 'static>(&self) -> impl Scope<'scope, (ImportedItem<T>, Items)>
    where
        (ImportedItem<T>, Items): ScopeItems;

    fn import_owned<T: 'static>(&self) -> impl Scope<'scope, (ExportedItem<T>, Items)>
    where
        (ExportedItem<T>, Items): ScopeItems;

    fn resolve(&self) -> Result<<Items as ScopeItems>::Unpack, UnknownItemError>;

    fn len(&self) -> usize;

    #[cfg(test)]
    fn forget_items(&self) -> impl Scope<()>;
}

impl<'scope, Items: ScopeItems> Scope<'scope, Items> for RootScope {
    fn export<T: 'static>(&mut self, value: T) -> impl Scope<'scope, (ExportedItem<T>, Items)>
    where
        (ExportedItem<T>, Items): ScopeItems,
    {
        let mut payload = self.payload.borrow_mut();
        if payload.error.is_none() {
            match payload.items.insert::<StoredItem<T>>(value) {
                Ok(_) => {}
                Err(msg) => payload.error = Some(UnknownItemError(msg)),
            }
        }

        RootScope {
            payload: self.payload.clone(),
        }
    }

    fn import<T: 'static>(&self) -> impl Scope<'scope, (ImportedItem<T>, Items)>
    where
        (ImportedItem<T>, Items): ScopeItems,
    {
        RootScope {
            payload: self.payload.clone(),
        }
    }

    fn import_owned<T: 'static>(&self) -> impl Scope<'scope, (ExportedItem<T>, Items)>
    where
        (ExportedItem<T>, Items): ScopeItems,
    {
        RootScope {
            payload: self.payload.clone(),
        }
    }

    fn resolve(&self) -> Result<<Items as ScopeItems>::Unpack, UnknownItemError> {
        let payload = self.payload.borrow();
        match &payload.error {
            Some(err) => Err(err.clone()),
            None => Ok(<Items as ScopeItems>::resolve(&payload.items)),
        }
    }

    fn len(&self) -> usize {
        self.payload.borrow().items.len()
    }

    #[cfg(test)]
    fn forget_items(&self) -> impl Scope<()> {
        RootScope {
            payload: self.payload.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownItemError(pub String);

pub struct ImportedItem<T: 'static>(T);

pub struct ExportedItem<T: 'static>(T);

pub(crate) struct StoredItem<T: 'static>(T);

impl<T: 'static> PolyMapKey for StoredItem<T> {
    type PolyMapValue = T;
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use super::accessors::{ReadOnly, ReadWrite};
    use super::{RootScope, Scope};

    #[test]
    fn basic() {
        let mut scope = RootScope::new();

        #[derive(Debug, PartialEq, Eq)]
        struct MyBool(bool);

        #[derive(Debug, PartialEq, Eq)]
        struct MyString(String);

        let scope = &scope
            .export(MyBool(true))
            .export(MyString("OH HAI!".into()));

        let (my_bool, my_string) = scope.resolve().unwrap();

        assert_eq!(my_bool, ReadWrite::new(Rc::new(RefCell::new(MyBool(true)))));
        assert_eq!(
            my_string,
            ReadWrite::new(Rc::new(RefCell::new(MyString("OH HAI!".into()))))
        );
    }

    #[test]
    fn export_then_import() {
        let mut root_scope = RootScope::new();

        #[derive(Debug, PartialEq, Eq)]
        struct MyBool(bool);

        #[derive(Debug, PartialEq, Eq)]
        struct MyString(String);

        let scope = root_scope
            .export(MyBool(true))
            .export(MyString("OH HAI!".into()));

        let scope = scope.forget_items();

        let scope = scope.import::<MyBool>().import::<MyString>();

        let (my_bool, my_string) = scope.resolve().unwrap();

        assert_eq!(my_bool, ReadOnly::new(Rc::new(RefCell::new(MyBool(true)))));
        assert_eq!(
            my_string,
            ReadOnly::new(Rc::new(RefCell::new(MyString("OH HAI!".into()))))
        );
    }

    #[test]
    fn items_are_resolvable_after_scope_clone() {
        let mut scope = RootScope::new();

        #[derive(Clone, Debug, PartialEq, Eq)]
        struct MyBool(bool);

        #[derive(Clone, Debug, PartialEq, Eq)]
        struct MyString(String);

        let scope = scope
            .export(MyBool(true))
            .export(MyString("OH HAI!".into()));

        let scope = scope.clone();

        let (my_bool, my_string) = scope.resolve().unwrap();

        assert_eq!(my_bool, ReadWrite::new(Rc::new(RefCell::new(MyBool(true)))));
        assert_eq!(
            my_string,
            ReadWrite::new(Rc::new(RefCell::new(MyString("OH HAI!".into()))))
        );
    }
}
