/*
    Appellation: transition <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::builder::TransitionBuilder;
use crate::prelude::{Direction, State};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Transition<T = String> {
    pub(crate) current_state: State<T>,
    pub(crate) read_symbol: char,
    pub(crate) next_state: State<T>,
    pub(crate) write_symbol: char,
    pub(crate) direction: Direction,
}

impl<T> Transition<T> {
    pub fn new(initial_state: State<T>) -> TransitionBuilder<T>
    where
        T: Default,
    {
        TransitionBuilder::new(initial_state)
    }

    pub const fn current_state(&self) -> &State<T> {
        &self.current_state
    }

    pub fn read_symbol(&self) -> char {
        self.read_symbol
    }

    pub const fn next_state(&self) -> &State<T> {
        &self.next_state
    }

    pub fn write_symbol(&self) -> char {
        self.write_symbol
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }
}

mod builder {
    use super::*;

    pub struct TransitionBuilder<T = String> {
        current_state: State<T>,
        read_symbol: char,
        next_state: State<T>,
        write_symbol: char,
        direction: Direction,
    }

    impl<T> TransitionBuilder<T> {
        pub fn new(initial_state: State<T>) -> Self
        where
            T: Default,
        {
            Self {
                current_state: initial_state,
                read_symbol: '\0',
                next_state: State::default(),
                write_symbol: '\0',
                direction: Direction::Right,
            }
        }

        pub fn read_symbol(mut self, read_symbol: char) -> Self {
            self.read_symbol = read_symbol;
            self
        }

        pub fn next_state(mut self, next_state: State<T>) -> Self {
            self.next_state = next_state;
            self
        }

        pub fn write_symbol(mut self, write_symbol: char) -> Self {
            self.write_symbol = write_symbol;
            self
        }

        pub fn direction(mut self, direction: Direction) -> Self {
            self.direction = direction;
            self
        }

        pub fn build(self) -> Transition<T> {
            Transition {
                current_state: self.current_state,
                read_symbol: self.read_symbol,
                next_state: self.next_state,
                write_symbol: self.write_symbol,
                direction: self.direction,
            }
        }
    }
}
