/*
    Appellation: chord <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{chord::Chord, dyad::Dyad};

pub(crate) mod chord;
pub(crate) mod dyad;

pub(crate) mod prelude {
    pub use super::chord::*;
    pub use super::dyad::*;
}

pub trait LinearData {
    type Elem;

    fn get(&self, idx: usize) -> &Self::Elem;

    fn len(&self) -> usize;
}

pub trait Container {
    type Elem;
}

impl<T> Container for Vec<T> {
    type Elem = T;
}

/// [ChordData] provides common methods for viable representations of chords.
/// Typically, implementations of [ChordData] describe linear data-structures
/// such as arrays, vectors, or slices.
pub trait ChordData {
    type Elem;

    fn get(&self, idx: usize) -> Option<&Self::Elem>;

    fn len(&self) -> usize;

    fn push(&mut self, elem: Self::Elem);
}
