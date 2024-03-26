//! Module of convenience functions to return various `VisitorControlFlow`
//! values, and combinators for chaining calls to the `Visitor` trait.
use super::*;

/// Shorthand for returning a  `VisitorControlFlow::Continue(state)`
pub fn cont<'ast, State>(state: State) -> VisitorControlFlow<'ast, State> {
    VisitorControlFlow::Continue(state)
}

/// Shorthand for modifying state and then returning a
/// `VisitorControlFlow::Continue(state)`
pub fn modify_state<'ast, State, F, N>(
    f: F,
) -> impl Fn(&'ast N, State) -> VisitorControlFlow<'ast, State>
where
    N: Visitable<'ast>,
    F: Fn(&mut State),
{
    move |_: &'ast N, mut state: State| {
        f(&mut state);
        flow::cont(state)
    }
}

/// Shorthand for returning a
/// `VisitorControlFlow::Break(Break::SkipChildren(state))`
pub fn skip<'ast, State>(state: State) -> VisitorControlFlow<'ast, State> {
    VisitorControlFlow::Break(Break::SkipChildren(state))
}

/// Shorthand for returning a
/// `VisitorControlFlow::Break(Break::Abort(state))`
pub fn abort<'ast, State>(state: State) -> VisitorControlFlow<'ast, State> {
    VisitorControlFlow::Break(Break::Finished(state))
}

/// Shorthand for returning a
/// `VisitorControlFlow::Break(Break::Err(Box::new(error), state))`
pub fn error<'ast, State, E: Error + 'ast>(
    error: E,
    state: State,
) -> VisitorControlFlow<'ast, State> {
    VisitorControlFlow::Break(Break::Err(Box::new(error), state))
}

/// Combinator to chain calls to [`Visitor::enter`] or [`Visitor::exit`]
/// when the previous invocation returned
/// `VisitorControlFlow::Continue(state)`, otherwise immediately returns the
/// break value.
pub fn map_continue<'ast, State, F>(
    flow: VisitorControlFlow<'ast, State>,
    visit_next: F,
) -> VisitorControlFlow<'ast, State>
where
    F: FnOnce(State) -> VisitorControlFlow<'ast, State>,
{
    match flow {
        VisitorControlFlow::Continue(state) => visit_next(state),
        brk => brk,
    }
}
