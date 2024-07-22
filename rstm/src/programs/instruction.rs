/*
    Appellation: instruction <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::builder::InstructionBuilder;

use crate::prelude::{Direction, State};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Instruction<T = String> {
    pub(crate) direction: Direction,
    pub(crate) state: State<T>,
    pub(crate) symbol: char,
    pub(crate) next_state: State<T>,
    pub(crate) write_symbol: char,
}

impl<T> Instruction<T> {
    pub fn new() -> InstructionBuilder<T> {
        InstructionBuilder::new()
    }

    pub fn state(&self) -> &State<T> {
        &self.state
    }

    pub fn symbol(&self) -> char {
        self.symbol
    }

    pub fn next_state(&self) -> &State<T> {
        &self.next_state
    }

    pub fn write_symbol(&self) -> char {
        self.write_symbol
    }
}

mod builder {
    use super::*;

    pub struct InstructionBuilder<Q> {
        pub(crate) direction: Direction,
        pub(crate) state: Option<State<Q>>,
        pub(crate) symbol: Option<char>,
        pub(crate) next_state: Option<State<Q>>,
        pub(crate) write_symbol: Option<char>,
    }

    impl<Q> InstructionBuilder<Q> {
        pub fn new() -> Self {
            Self {
                direction: Direction::Right,
                state: None,
                symbol: None,
                next_state: None,
                write_symbol: None,
            }
        }

        pub fn direction(self, direction: Direction) -> Self {
            Self { direction, ..self }
        }

        pub fn left(self) -> Self {
            self.direction(Direction::Left)
        }

        pub fn state(self, state: State<Q>) -> Self {
            Self {
                state: Some(state),
                ..self
            }
        }

        pub fn symbol(self, symbol: char) -> Self {
            Self {
                symbol: Some(symbol),
                ..self
            }
        }

        pub fn next_state(self, next_state: State<Q>) -> Self {
            Self {
                next_state: Some(next_state),
                ..self
            }
        }

        pub fn write_symbol(self, write_symbol: char) -> Self {
            Self {
                write_symbol: Some(write_symbol),
                ..self
            }
        }

        pub fn build(self) -> Instruction<Q> {
            Instruction {
                direction: self.direction,
                state: self.state.expect("state is required"),
                symbol: self.symbol.expect("symbol is required"),
                next_state: self.next_state.expect("next_state is required"),
                write_symbol: self.write_symbol.expect("write_symbol is required"),
            }
        }
    }
}
