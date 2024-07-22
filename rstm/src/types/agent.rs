/*
    Appellation: agent <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Head, Tail};
use crate::prelude::{Direction, State};

pub struct Agent<T> {
    pub(crate) head: Head<T>,
    pub(crate) tail: Tail<T>,
}

impl<T> Agent<T> {
    pub fn new(head: Head<T>, tail: Tail<T>) -> Self {
        Self { head, tail }
    }

    pub const fn head(&self) -> &Head<T> {
        &self.head
    }

    pub const fn tail(&self) -> &Tail<T> {
        &self.tail
    }

    pub fn head_mut(&mut self) -> &mut Head<T> {
        &mut self.head
    }
    /// Returns a mutable reference to the tail of the agent
    pub fn tail_mut(&mut self) -> &mut Tail<T> {
        &mut self.tail
    }
    /// Mutably shift the agent in the specified [direction](Direction)
    pub fn move_head(&mut self, direction: Direction) {
        self.head.move_head(direction);
    }
    /// Returns the current [state](State) the agent is in
    pub fn current_state(&self) -> &State<T> {
        self.head.state()
    }
    /// Returns a copy of the [direction](Direction) the agent is instructed to move
    pub fn direction(&self) -> Direction {
        self.tail().direction()
    }
    /// Returns an immutable, owned reference to the next [state](State)
    pub fn next_state(&self) -> &State<T> {
        self.tail().next_state()
    }
    /// Returns a copy of the symbol the agent is instructed to write
    pub fn write_symbol(&self) -> char {
        self.tail().write_symbol
    }

    pub fn step(self) -> Self {
        Self {
            head: self.head.step(self.tail.direction),
            ..self
        }
    }
}
