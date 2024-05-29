use std::{convert::Infallible, fmt::Debug, marker::PhantomData};

use sqltk_core::Visitable;
use sqltk_derive::Visitor;

#[test]
fn derive_visitor_for_struct() {
    #[derive(Visitor)]
    #[visitor(
            state_ty = bool,
            error_ty = Infallible,
            enter = { self.continue_with_state(!state) },
            exit = { self.continue_with_state(state) },
        )]
    struct MyVisitor;

    assert_eq!(String::from("FOO").evaluate(&MyVisitor, false), Ok(true));
}

#[test]
fn derive_visitor_for_struct_with_state_bounds() {
    #[derive(Visitor)]
    #[visitor(
            error_ty = Infallible,
            enter = { self.continue_with_state(state) },
            exit = { self.continue_with_state(state) },
        )]
    struct MyVisitor<State>(PhantomData<State>)
    where
        State: Debug;

    assert_eq!(
        String::from("FOO").evaluate(&MyVisitor(PhantomData), true),
        Ok(true)
    );
}

#[test]
fn derive_visitor_for_struct_without_error_ty() {
    #[derive(Visitor)]
    #[visitor(
            enter = { self.continue_with_state(state) },
            exit = { self.continue_with_state(state) },
        )]
    struct MyVisitor<State>(PhantomData<State>)
    where
        State: Debug;

    assert_eq!(
        String::from("FOO").evaluate(&MyVisitor(PhantomData), true),
        Ok(true)
    );
}

#[test]
fn derive_visitor_for_enum() {
    #[derive(Visitor)]
    #[visitor(
            state_ty = bool,
            error_ty = Infallible,
            enter = { self.continue_with_state(!state) },
        )]
    struct MyVisitor1;

    #[derive(Visitor)]
    #[visitor(
            state_ty = bool,
            error_ty = Infallible,
            enter = { self.continue_with_state(state) },
        )]
    struct MyVisitor2;

    #[derive(Visitor)]
    #[visitor(
            state_ty = bool,
            error_ty = Infallible,
        )]
    enum MyEnumVisitor {
        Visitor1(MyVisitor1),
        Visitor2(MyVisitor2),
    }

    assert_eq!(
        String::from("FOO").evaluate(&MyEnumVisitor::Visitor1(MyVisitor1), false),
        Ok(true)
    );
    assert_eq!(
        String::from("FOO").evaluate(&MyEnumVisitor::Visitor2(MyVisitor2), false),
        Ok(false)
    );
}

#[test]
fn derive_visitor_for_enum_with_state_bounds() {
    #[derive(Visitor)]
    #[visitor(
            error_ty = Infallible,
            enter = { self.continue_with_state(state) },
        )]
    struct MyVisitor1<State>(PhantomData<State>)
    where
        State: Debug;

    #[derive(Visitor)]
    #[visitor(
            error_ty = Infallible,
        )]
    enum MyEnumVisitor<State>
    where
        State: Debug,
    {
        Visitor1(MyVisitor1<State>),
    }

    assert_eq!(
        String::from("FOO").evaluate(&MyEnumVisitor::Visitor1(MyVisitor1(PhantomData)), false),
        Ok(false)
    );
}
