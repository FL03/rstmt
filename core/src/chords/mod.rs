pub use self::chord::*;

pub(crate) mod chord;

pub(crate) mod prelude {
    pub use super::chord::*;
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

    fn get(&self, idx: usize) -> &Self::Elem;

    fn len(&self) -> usize;

    fn push(&mut self, elem: Self::Elem);
}
