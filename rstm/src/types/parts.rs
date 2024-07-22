/*
    Appellation: head <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{Direction, State};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Head<T> {
    pub(crate) position: usize,
    pub(crate) state: State<T>,
}

impl<T> Head<T> {
    pub fn new(position: usize, state: State<T>) -> Self {
        Self { position, state }
    }

    pub fn move_head(&mut self, direction: Direction) {
        self.position = direction.apply(self.position);
    }

    pub fn step(self, direction: Direction) -> Self {
        Self {
            position: direction.apply(self.position),
            ..self
        }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub const fn state(&self) -> &State<T> {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut State<T> {
        &mut self.state
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Tail<T = String> {
    pub(crate) next_state: State<T>,
    pub(crate) write_symbol: char,
    pub(crate) direction: Direction,
}

impl<T> Tail<T> {
    pub fn new(next_state: State<T>, write_symbol: char, direction: Direction) -> Self {
        Self {
            next_state,
            write_symbol,
            direction,
        }
    }

    pub fn next_state(&self) -> &State<T> {
        &self.next_state
    }

    pub fn write_symbol(&self) -> char {
        self.write_symbol
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }
}
