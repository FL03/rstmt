/*
    Appellation: impl_chord_base <module>
    Created At: 2026.01.20:08:26:02
    Contrib: @FL03
*/
use crate::chords::chord_base::ChordBase;

use crate::chords::RawChord;

impl<S, T> ChordBase<S, T>
where
    S: RawChord<Elem = T>,
{
    /// creates a new [`ChordBase`] instance from a given chord representation.
    pub const fn new(repr: S) -> Self {
        Self { repr }
    }
    /// returns a reference to the underlying chord representation.
    pub const fn data(&self) -> &S {
        &self.repr
    }
    /// returns a mutable reference to the underlying chord representation.
    pub const fn data_mut(&mut self) -> &mut S {
        &mut self.repr
    }
    /// returns the number of elements in the chord representation.
    pub fn len(&self) -> usize {
        self.repr.len()
    }
    /// returns true if the chord contains no elements
    pub fn is_empty(&self) -> bool {
        self.repr.is_empty()
    }
}
