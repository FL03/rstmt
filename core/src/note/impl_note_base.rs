/*
    Appellation: impl_note_base <module>
    Created At: 2025.12.20:09:35:09
    Contrib: @FL03
*/
use super::NoteBase;
use crate::octave::Octave;
use crate::pitch::{Accidental, PitchClass, PitchClassRepr, RawAccidental, RawPitchClass};

impl<P, K> NoteBase<P, K>
where
    P: RawPitchClass<Tag = K>,
    K: RawAccidental,
{
    /// constructs a new [`NoteBase`] instance
    pub const fn new(class: PitchClass<P, K>, octave: Octave) -> Self {
        Self { class, octave }
    }
    /// initialize a new instance of the note from the given octave
    pub fn from_octave(octave: Octave) -> Self
    where
        P: PitchClassRepr,
        K: Accidental,
    {
        Self {
            class: PitchClass::new(),
            octave,
        }
    }
    /// returns a reference to the current class
    pub const fn class(&self) -> &PitchClass<P, K> {
        &self.class
    }
    /// returns a mutable reference to the current class
    pub const fn class_mut(&mut self) -> &mut PitchClass<P, K> {
        &mut self.class
    }
    /// returns a reference to the current octave
    pub const fn octave(&self) -> &Octave {
        &self.octave
    }
    /// returns a mutable reference to the current octave
    pub const fn octave_mut(&mut self) -> &mut Octave {
        &mut self.octave
    }
    /// returns string formatted following the American Standard Pitch Notation (ASPN) of:
    /// "C.4", "D#.5", etc.
    pub fn aspn(&self) -> String {
        format!("{}.{}", self.class().name(), self.octave().value())
    }
}
