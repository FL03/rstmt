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

#[doc(hidden)]
pub trait Container {
    type Elem;
}

#[doc(hidden)]
pub trait BaseData {
    type Elem;

    private!();

    #[doc(hidden)]
    fn as_slice(&self) -> &[Self::Elem];

    fn is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }

    fn len(&self) -> usize {
        self.as_slice().len()
    }
}

/// [ChordData] provides common methods for viable representations of chords.
/// Typically, implementations of [ChordData] describe linear data-structures
/// such as arrays, vectors, or slices.
pub trait ChordData: BaseData {
    fn get(&self, idx: usize) -> Option<&Self::Elem>;

    fn push(&mut self, elem: Self::Elem);
}

/*
    ************* Implementations *************
*/
impl<T> Container for Vec<T> {
    type Elem = T;
}
