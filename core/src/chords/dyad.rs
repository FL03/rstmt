/*
    Appellation: dyad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::notes::Note;
use crate::{Intervals, Pair};

fn _interval<A, B>(lhs: A, rhs: B) -> Intervals
where
    A: core::ops::Sub<B, Output = Intervals>,
{
    lhs - rhs
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Dyad {
    chord: Pair<Note>,
    interval: Intervals,
}

impl Dyad {
    pub fn new(src: Note, tgt: Note) -> Self {
        let chord = Pair::new(src, tgt);
        let interval = Intervals::new(src, tgt);
        Self { chord, interval }
    }

    pub fn from_tuple((lhs, rhs): (Note, Note)) -> Self {
        let chord = Pair::new(lhs, rhs);
        let interval = Intervals::new(lhs, rhs);
        Self { chord, interval }
    }

    pub const fn chord(&self) -> &Pair<Note> {
        &self.chord
    }

    pub fn chord_mut(&mut self) -> &mut Pair<Note> {
        &mut self.chord
    }

    pub const fn interval(&self) -> &Intervals {
        &self.interval
    }

    pub fn interval_mut(&mut self) -> &mut Intervals {
        &mut self.interval
    }
}
