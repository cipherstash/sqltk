//! Module of convenience functions to return various `VisitorControlFlow`
//! values, and combinators for chaining calls to the `Visitor` trait.
use super::*;

/// Shorthand for returning a  `VisitorControlFlow::Continue(state)`
pub fn cont<'ast, State, E>(state: State) -> VisitorControlFlow<'ast, State, E> {
    VisitorControlFlow::Continue(state)
}

/// Shorthand for modifying state and then returning a
/// `VisitorControlFlow::Continue(state)`
pub fn modify_state<'ast, State, E, F, N>(
    f: F,
) -> impl Fn(&'ast N, State) -> VisitorControlFlow<'ast, State, E>
where
    E: Error + Debug,
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
pub fn skip<'ast, State, E>(state: State) -> VisitorControlFlow<'ast, State, E>
where
    E: Error + Debug,
{
    VisitorControlFlow::Break(Break::SkipChildren(state))
}

/// Shorthand for returning a
/// `VisitorControlFlow::Break(Break::Abort(state))`
pub fn finished<'ast, State, E>(state: State) -> VisitorControlFlow<'ast, State, E>
where
    E: Error + Debug,
{
    VisitorControlFlow::Break(Break::Finished(state))
}

/// Shorthand for returning a
/// `VisitorControlFlow::Break(Break::Err(error))`
pub fn error<'ast, State, E>(error: E) -> VisitorControlFlow<'ast, State, E>
where
    E: Error + Debug,
{
    VisitorControlFlow::Break(Break::Err(error))
}

/// Chains calls to [`Visitor::enter`] or [`Visitor::exit`]
/// when the previous invocation returned
/// `VisitorControlFlow::Continue(state)`, otherwise immediately returns the
/// break value.
pub fn map_continue<'ast, State, E, F>(
    flow: VisitorControlFlow<'ast, State, E>,
    visit_next: F,
) -> VisitorControlFlow<'ast, State, E>
where
    E: Error + Debug,
    F: FnOnce(State) -> VisitorControlFlow<'ast, State, E>,
{
    match flow {
        VisitorControlFlow::Continue(state) => visit_next(state),
        brk => brk,
    }
}

/// Converts errors from one type to another if there is an error contained in
/// the [`VisitorControlFlow`] value, otherwise returns the value unchanged.
pub fn map_break_err<State, ErrFrom, ErrInto>(
    flow: VisitorControlFlow<'_, State, ErrFrom>,
) -> VisitorControlFlow<'_, State, ErrInto>
where
    ErrFrom: Error + Debug + Into<ErrInto>,
    ErrInto: Error + Debug,
{
    match flow {
        VisitorControlFlow::Break(Break::Err(err_from)) => {
            VisitorControlFlow::Break(Break::Err(err_from.into()))
        }
        VisitorControlFlow::Break(Break::SkipChildren(state)) => {
            VisitorControlFlow::Break(Break::SkipChildren(state))
        }
        VisitorControlFlow::Break(Break::Finished(state)) => {
            VisitorControlFlow::Break(Break::Finished(state))
        }
        VisitorControlFlow::Continue(other) => VisitorControlFlow::Continue(other),
    }
}
