use super::*;

/// Trait of convenience functions to return various `ControlFlow`
/// values, and combinators for chaining calls to the `Visitor` trait.
///
/// This module has a blanket implementation for all Visitors.
pub trait VisitorExtensions<'ast, State, Err: Error + Debug> {
    /// Shorthand for returning a  `ControlFlow::Continue(state)`
    fn continue_with_state(&self, state: State) -> ControlFlow<Break<State, Err>, State> {
        ControlFlow::Continue(state)
    }

    /// Shorthand for modifying state and then returning a
    /// `ControlFlow::Continue(state)`
    fn continue_with_modified_state<F, N>(
        &self,
        f: F,
    ) -> impl Fn(&'ast N, State) -> ControlFlow<Break<State, Err>, State>
    where
        N: Visitable<'ast>,
        F: Fn(&mut State),
    {
        move |_: &'ast N, mut state: State| {
            f(&mut state);
            self.continue_with_state(state)
        }
    }

    /// Shorthand for returning a
    /// `ControlFlow::Break(Break::SkipChildren(state))`
    fn continue_but_skip_children(&self, state: State) -> ControlFlow<Break<State, Err>, State> {
        ControlFlow::Break(Break::SkipChildren(state))
    }

    /// Shorthand for returning a
    /// `ControlFlow::Break(Break::Abort(state))`
    fn break_finished_with_state(&self, state: State) -> ControlFlow<Break<State, Err>, State> {
        ControlFlow::Break(Break::Finished(state))
    }

    /// Shorthand for returning a
    /// `ControlFlow::Break(Break::Err(error))`
    fn break_with_error(&self, error: Err) -> ControlFlow<Break<State, Err>, State> {
        ControlFlow::Break(Break::Err(error))
    }

    /// Chains calls to [`Visitor::enter`] or [`Visitor::exit`]
    /// when the previous invocation returned
    /// `ControlFlow::Continue(state)`, otherwise immediately returns the
    /// break value.
    fn map_continue<F>(
        &self,
        control_flow: ControlFlow<Break<State, Err>, State>,
        next_op: F,
    ) -> ControlFlow<Break<State, Err>, State>
    where
        F: FnOnce(State) -> ControlFlow<Break<State, Err>, State>,
    {
        match control_flow {
            ControlFlow::Continue(state) => next_op(state),
            brk => brk,
        }
    }

    /// Converts errors from one type to another if there is an error contained in
    /// the [`ControlFlow`] value, otherwise returns the value unchanged.
    fn map_break_error<ErrFrom>(
        &self,
        control_flow: ControlFlow<Break<State, ErrFrom>, State>,
    ) -> ControlFlow<Break<State, Err>, State>
    where
        ErrFrom: Error + Debug + Into<Err>,
    {
        match control_flow {
            ControlFlow::Break(Break::Err(err_from)) => {
                ControlFlow::Break(Break::Err(err_from.into()))
            }
            ControlFlow::Break(Break::SkipChildren(state)) => {
                ControlFlow::Break(Break::SkipChildren(state))
            }
            ControlFlow::Break(Break::Finished(state)) => {
                ControlFlow::Break(Break::Finished(state))
            }
            ControlFlow::Continue(other) => ControlFlow::Continue(other),
        }
    }
}

impl<'ast, V: Visitor<'ast>> VisitorExtensions<'ast, V::State, V::Error> for V {}
