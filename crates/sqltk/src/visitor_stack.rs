use core::marker::PhantomData;
use std::fmt::{self, Debug, Formatter};
use std::{any::type_name, error::Error};

use crate::{flow, Break, Node, Visitor, VisitorControlFlow};

include!(concat!(
    env!("OUT_DIR"),
    "/generated_node_enum_match_macro.rs"
));

/// Composes multiple [`Visitor`] implementations into a single `Visitor`.
///
/// All `Visitor` implementations that are pushed onto the stack must have an
/// error type that satisifies `ErrorConversion<VE>: Into<E>`.
///
/// `VisitorStack` implements `Visitor`.
///
/// [`Visitor::enter`] is implemented such that `enter` is called on the child
/// visitors in the order they were pushed onto the stack.
///
/// For [`Visitor::exit`] the order is reversed.
pub struct VisitorStack<'ast, State, E> {
    visitors: Vec<Box<dyn ObjectSafeVisitor<'ast, State, E> + 'ast>>,
}

impl<'ast, State, E> Debug for VisitorStack<'ast, State, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let visitor_names: Vec<String> = self.visitors.iter().map(|v| v.inner_visitor_type_name() ).collect();
        f.debug_struct(type_name::<Self>())
            .field("visitors", &visitor_names)
            .finish()
    }
}

impl<'ast, State, E> Default for VisitorStack<'ast, State, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'ast, State, E> VisitorStack<'ast, State, E> {
    /// Creates a new empty `VisitorStack`.
    ///
    /// This allocates capacity for 16 visitors which will grow on demand. A
    /// [`Vec`] is used for internal storage.
    pub fn new() -> Self {
        Self {
            visitors: Vec::with_capacity(32),
        }
    }
}

impl<'ast, State, E> VisitorStack<'ast, State, E>
where
    E: 'ast + Error,
    State: 'ast,
{
    /// Pushes a new [`Visitor`] onto this `VisitorStack`.
    pub fn push<V, VE>(&mut self, visitor: V)
    where
        V: 'ast + Visitor<'ast, State, VE>,
        VE: 'ast + Error,
        ConvertFrom<VisitorControlFlow<'ast, State, VE>>:
            Into<ConvertInto<VisitorControlFlow<'ast, State, E>>>,
    {
        self.visitors
            .push(Box::new(ErrorConvertingVisitor::new(visitor))
                as Box<dyn ObjectSafeVisitor<'ast, State, E> + 'ast>);
    }
}

pub struct ConvertFrom<T>(T);
pub struct ConvertInto<T>(T);

impl<T> ConvertInto<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<'ast, State, EFrom, EInto> From<ConvertFrom<VisitorControlFlow<'ast, State, EFrom>>>
    for ConvertInto<VisitorControlFlow<'ast, State, EInto>>
where
    EFrom: Into<EInto>,
{
    fn from(value: ConvertFrom<VisitorControlFlow<'ast, State, EFrom>>) -> Self {
        match value.0 {
            VisitorControlFlow::Continue(state) => ConvertInto(VisitorControlFlow::Continue(state)),
            VisitorControlFlow::Break(brk) => match brk {
                Break::SkipChildren(state) => {
                    ConvertInto(VisitorControlFlow::Break(Break::SkipChildren(state)))
                }
                Break::Finished(state) => {
                    ConvertInto(VisitorControlFlow::Break(Break::Finished(state)))
                }
                Break::Err(err, state) => {
                    ConvertInto(VisitorControlFlow::Break(Break::Err(err.into(), state)))
                }
            },
        }
    }
}

impl<'ast, State, E> Visitor<'ast, State, E> for VisitorStack<'ast, State, E> {
    fn enter<N: 'static>(&self, node: &'ast N, mut state: State) -> VisitorControlFlow<'ast, State, E>
    where
        &'ast N: Into<Node<'ast>>,
    {
        for visitor in self.visitors.iter() {
            state = visitor.object_safe_enter(node.into(), state)?;
        }
        flow::cont(state)
    }

    fn exit<N: 'static>(&self, node: &'ast N, mut state: State) -> VisitorControlFlow<'ast, State, E>
    where
        &'ast N: Into<Node<'ast>>,
    {
        for visitor in self.visitors.iter().rev() {
            state = visitor.object_safe_exit(node.into(), state)?;
        }
        flow::cont(state)
    }
}

/// A [`Visitor`] implementation that converts between error types.
struct ErrorConvertingVisitor<'ast, State, E, V, VE>
where
    ConvertFrom<VisitorControlFlow<'ast, State, VE>>:
        Into<ConvertInto<VisitorControlFlow<'ast, State, E>>>,
{
    visitor: V,
    _phantom: PhantomData<(&'ast (), State, E, VE)>,
}

impl<'ast, State, E, V, VE> ErrorConvertingVisitor<'ast, State, E, V, VE>
where
    E: Error,
    VE: Error,
    ConvertFrom<VisitorControlFlow<'ast, State, VE>>:
        Into<ConvertInto<VisitorControlFlow<'ast, State, E>>>,
{
    fn new(visitor: V) -> Self {
        Self {
            visitor,
            _phantom: PhantomData,
        }
    }
}

impl<'ast, State, E, V, VE> ObjectSafeVisitor<'ast, State, E>
    for ErrorConvertingVisitor<'ast, State, E, V, VE>
where
    V: Visitor<'ast, State, VE>,
    E: Error,
    VE: Error,
    ConvertFrom<VisitorControlFlow<'ast, State, VE>>:
        Into<ConvertInto<VisitorControlFlow<'ast, State, E>>>,
{
    fn object_safe_enter(
        &self,
        node: Node<'ast>,
        state: State,
    ) -> VisitorControlFlow<'ast, State, E> {
        let result = dispatch_node!(node, self.visitor, enter, state);
        ConvertFrom(result).into().into_inner()
    }

    fn object_safe_exit(
        &self,
        node: Node<'ast>,
        state: State,
    ) -> VisitorControlFlow<'ast, State, E> {
        let result = dispatch_node!(node, self.visitor, exit, state);
        ConvertFrom(result).into().into_inner()
    }

    fn inner_visitor_type_name(&self) -> String {
        type_name::<V>().into()
    }
}

/// Variation of the [`Visitor`] trait where the node is passed as an enum
/// instead of as a generically typed reference. This trait is therefore object
/// safe.
trait ObjectSafeVisitor<'ast, State, E> {
    fn object_safe_enter(
        &self,
        node: Node<'ast>,
        state: State,
    ) -> VisitorControlFlow<'ast, State, E>;

    fn object_safe_exit(
        &self,
        node: Node<'ast>,
        state: State,
    ) -> VisitorControlFlow<'ast, State, E>;

    fn inner_visitor_type_name(&self) -> String;
}

impl<'ast, State, E> Debug for dyn ObjectSafeVisitor<'ast, State, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let type_name = self.inner_visitor_type_name();
        f.debug_struct("dyn ObjectSafeVisitor")
            .field("inner_visitor", &type_name)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use core::convert::Infallible;

    use crate::{flow, Visitable, VisitorStack};

    #[test]
    fn basic() {
        let ast = "OH HAI!".to_owned();

        let mut stack = VisitorStack::<usize, Infallible>::new();

        stack.push(String::on_enter(|_, mut state| {
            state += 1;
            flow::infallible::cont(state)
        }));

        match ast.evaluate(&stack, 0) {
            Ok(result) => assert_eq!(result, 1),
            Err(_) => panic!("Infallible!"),
        };
    }
}
