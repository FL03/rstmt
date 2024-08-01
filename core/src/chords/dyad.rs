/*
    Appellation: dyad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::notes::Note;
use crate::{Interval, Pair};

fn _interval<A, B>(lhs: A, rhs: B) -> Interval
where
    A: core::ops::Sub<B, Output = Interval>,
{
    lhs - rhs
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Dyad {
    chord: Pair<Note>,
    interval: Interval,
}

impl Dyad {
    pub fn new(src: Note, tgt: Note) -> Self {
        let chord = Pair::new(src, tgt);
        let interval = Interval::new(src, tgt);
        Self { chord, interval }
    }

    pub fn from_tuple((lhs, rhs): (Note, Note)) -> Self {
        let chord = Pair::new(lhs, rhs);
        let interval = Interval::new(lhs, rhs);
        Self { chord, interval }
    }

    pub const fn chord(&self) -> &Pair<Note> {
        &self.chord
    }

    pub fn chord_mut(&mut self) -> &mut Pair<Note> {
        &mut self.chord
    }

    pub const fn interval(&self) -> &Interval {
        &self.interval
    }

    pub fn interval_mut(&mut self) -> &mut Interval {
        &mut self.interval
    }
}
