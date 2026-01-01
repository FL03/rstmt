/*
    Appellation: impl_note_base <module>
    Created At: 2025.12.20:09:35:09
    Contrib: @FL03
*/
use super::NoteBase;
use crate::octave::Octave;
use crate::pitch::{Accidental, PitchClass, RawPitchClass};

impl<P, K> NoteBase<P, K>
where
    P: RawPitchClass<Tag = K>,
    K: Accidental,
{
    /// constructs a new [`NoteBase`] instance
    pub const fn new(class: PitchClass<P, K>, octave: Octave) -> Self {
        Self { class, octave }
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
}
