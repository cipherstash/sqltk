use core::marker::PhantomData;

use crate::{flow, Node, Visitable, Visitor, VisitorControlFlow};

/// The end of a [`Cons`] list.
/// Used for representing a type-heterogenous list of `Visitor` implementations.
#[doc(hidden)]
#[derive(Debug)]
pub struct Nil;

/// A `Cons` cell.
/// Used for representing a type-heterogenous list of `Visitor` implementations.
#[doc(hidden)]
#[derive(Debug)]
pub struct Cons<H, T>(H, T);

impl<H, T> Cons<H, T> {
    fn head(&self) -> &H {
        &self.0
    }

    fn tail(&self) -> &T {
        &self.1
    }
}

/// A composition of multiple [`Visitor`] implementations.
///
/// A `VisitorStack` implements `Visitor`.
///
/// When `Visitor::enter` is called on a `VisitorStack`, `enter` is called on
/// the child visitors from head to tail.  When `Visitor::exit` is called on a
/// `VisitorStack`, `exit` is called on the child visitors from tail to head.
///
/// The "head" of a `VisitorStack` is the *last* `Visitor` that was pushed onto
/// the stack.  The "tail" of a `VisitorStack` is the *first* `Visitor` that was
/// pushed onto the stack.
#[derive(Debug)]
pub struct VisitorStack<'ast, State, Visitors = Nil>
where
    Visitors: Visitor<'ast, State>,
{
    visitors: Visitors,
    _phantom: PhantomData<&'ast State>,
}

impl<'ast, State> Default for VisitorStack<'ast, State, Nil> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'ast, State> VisitorStack<'ast, State, Nil> {
    /// Construct a new, empty `VisitorStack`.
    pub fn new() -> Self {
        Self {
            visitors: Nil,
            _phantom: PhantomData,
        }
    }
}

impl<'ast, State, Visitors> VisitorStack<'ast, State, Visitors>
where
    Visitors: Visitor<'ast, State>,
{
    pub fn push<V>(self, visitor: V) -> VisitorStack<'ast, State, Cons<V, Visitors>>
    where
        V: Visitor<'ast, State>,
    {
        VisitorStack {
            visitors: Cons(visitor, self.visitors),
            _phantom: PhantomData,
        }
    }
}

impl<'ast, State, Visitors> Visitor<'ast, State> for VisitorStack<'ast, State, Visitors>
where
    Visitors: Visitor<'ast, State>,
{
    fn enter<N>(&self, node: &'ast N, state: State) -> VisitorControlFlow<'ast, State>
    where
        N: 'static + Visitable<'ast>,
        &'ast N: Into<Node<'ast>>,
    {
        self.visitors.enter(node, state)
    }

    fn exit<N>(&self, node: &'ast N, state: State) -> VisitorControlFlow<'ast, State>
    where
        N: 'static + Visitable<'ast>,
        &'ast N: Into<Node<'ast>>,
    {
        self.visitors.exit(node, state)
    }
}

impl<'ast, State> Visitor<'ast, State> for Nil {
    fn enter<N>(&self, _: &'ast N, state: State) -> VisitorControlFlow<'ast, State>
    where
        N: 'static + Visitable<'ast>,
        &'ast N: Into<Node<'ast>>,
    {
        flow::cont(state)
    }

    fn exit<N>(&self, _: &'ast N, state: State) -> VisitorControlFlow<'ast, State>
    where
        N: 'static + Visitable<'ast>,
        &'ast N: Into<Node<'ast>>,
    {
        flow::cont(state)
    }
}

impl<'ast, State, H, T> Visitor<'ast, State> for Cons<H, T>
where
    H: Visitor<'ast, State>,
    T: Visitor<'ast, State>,
{
    fn enter<N>(&self, node: &'ast N, mut state: State) -> VisitorControlFlow<'ast, State>
    where
        N: 'static + Visitable<'ast>,
        &'ast N: Into<Node<'ast>>,
    {
        state = self.head().enter(node, state)?;
        self.tail().enter(node, state)
    }

    fn exit<N>(&self, node: &'ast N, mut state: State) -> VisitorControlFlow<'ast, State>
    where
        N: 'static + Visitable<'ast>,
        &'ast N: Into<Node<'ast>>,
    {
        state = self.tail().exit(node, state)?;
        self.head().exit(node, state)
    }
}

#[cfg(test)]
mod test {
    use crate::{flow, Visitable, VisitorControlFlow, VisitorStack};

    #[test]
    fn test_visitor_list_traversal_order() {
        let node = String::from("Blah");

        fn message<'ast, N: Visitable<'ast>>(
            string: &'static str,
        ) -> impl Fn(&'ast N, Vec<String>) -> VisitorControlFlow<'ast, Vec<String>> {
            |_, mut state: Vec<String>| -> VisitorControlFlow<'_, Vec<String>> {
                state.push(string.into());
                flow::cont(state)
            }
        }

        let stack = VisitorStack::new()
            .push(String::on_enter_exit(
                message("C::enter"),
                message("C::exit"),
            ))
            .push(String::on_enter_exit(
                message("B::enter"),
                message("B::exit"),
            ))
            .push(String::on_enter_exit(
                message("A::enter"),
                message("A::exit"),
            ));

        match node.evaluate(&stack, Vec::<String>::new()) {
            Ok(state) => {
                assert_eq!(
                    state,
                    vec!["A::enter", "B::enter", "C::enter", "C::exit", "B::exit", "A::exit",]
                )
            }
            Err(err) => assert!(false, "{:?}", err),
        };
    }
}
