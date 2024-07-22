/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{state::State, transition::Transition};

pub(crate) mod state;
pub(crate) mod transition;

pub(crate) mod prelude {
    pub use crate::state::{State, Transition};
}

pub trait States<T>: Clone + Default + Send + Sync {
    type Data: StateData<State = T>;
}

pub trait StateData {
    type State;
}

/// [Stateful] is used to describe objects which rely upon a state.
///
pub trait Stateful<Q = String> {
    type State: StateData<State = Q>;
}

impl<T> StateData for State<T> {
    type State = T;
}

pub enum TimePerspective<T> {
    Past(T),
    Present(State<T>),
    Future(State<T>),
}

pub struct StateSet<Q> {
    pub(crate) prev: State<Q>,
    pub(crate) curr: State<Q>,
    pub(crate) next: State<Q>,
}
