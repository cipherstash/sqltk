use core::marker::PhantomData;
use std::error::Error;
use std::fmt::Debug;

use crate::{flow, Break, Node, Visitable, Visitor, VisitorControlFlow};

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
pub struct VisitorStack<'ast, State, E>
where
    State: 'ast,
    E: 'static + Error + Debug,
{
    visitors: Vec<Box<dyn ObjectSafeVisitor<'ast, State, E> + 'ast>>,
}

impl<'ast, State, E> Default for VisitorStack<'ast, State, E>
where
    State: 'ast,
    E: 'static + Error + Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'ast, State, E> VisitorStack<'ast, State, E>
where
    State: 'ast,
    E: 'static + Error + Debug,
{
    /// Creates a new empty `VisitorStack`.
    ///
    /// This allocates capacity for 16 visitors which will grow on demand. A
    /// [`Vec`] is used for internal storage.
    pub fn new() -> Self {
        Self {
            visitors: Vec::with_capacity(16),
        }
    }

    /// Pushes a new [`Visitor`] onto this `VisitorStack`.
    pub fn push<V, VE>(&mut self, visitor: V)
    where
        V: 'ast + Visitor<'ast, State, VE>,
        VE: 'ast + Error + Debug,
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
    EFrom: Error + Into<EInto>,
    EInto: Error,
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

impl<'ast, State, E> Visitor<'ast, State, E> for VisitorStack<'ast, State, E>
where
    E: Error + Debug,
{
    fn enter<N>(&self, node: &'ast N, mut state: State) -> VisitorControlFlow<'ast, State, E>
    where
        N: 'static + Visitable<'ast>,
        &'ast N: Into<Node<'ast>>,
    {
        for visitor in self.visitors.iter() {
            state = visitor.object_safe_enter(node.into(), state)?;
        }
        flow::cont(state)
    }

    fn exit<N>(&self, node: &'ast N, mut state: State) -> VisitorControlFlow<'ast, State, E>
    where
        N: 'static + Visitable<'ast>,
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
    E: Error + Debug,
    VE: Error + Debug,
    V: Visitor<'ast, State, VE>,
    ConvertFrom<VisitorControlFlow<'ast, State, VE>>:
        Into<ConvertInto<VisitorControlFlow<'ast, State, E>>>,
{
    visitor: V,
    _phantom: PhantomData<(&'ast (), State, E, VE)>,
}

impl<'ast, State, E, V, VE> ErrorConvertingVisitor<'ast, State, E, V, VE>
where
    E: Error + Debug,
    VE: Error + Debug,
    V: Visitor<'ast, State, VE>,
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
    E: Error + Debug,
    VE: Error + Debug,
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
}

/// Variation of the [`Visitor`] trait where the node is passed as an enum
/// instead of as a generically typed reference. This trait is therefore object
/// safe.
trait ObjectSafeVisitor<'ast, State, E>
where
    E: Error + Debug,
{
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
