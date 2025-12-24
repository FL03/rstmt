/*
    Appellation: impl_note_base <module>
    Created At: 2025.12.20:09:35:09
    Contrib: @FL03
*/
use super::NoteBase;
use crate::octave::Octave;
use crate::pitch::{Pitch, PitchClassRepr};

impl<T, Cls> NoteBase<T, Cls>
where
    Cls: PitchClassRepr,
{
    pub fn new(pitch: Pitch<T>, octave: Octave) -> Self {
        Self {
            class: Cls::new(),
            octave,
            pitch,
        }
    }
    /// returns a copy to the index of the note's class
    pub const fn class(&self) -> &Cls {
        &self.class
    }
    /// returns a copy to the octave of the note
    pub const fn octave(&self) -> Octave {
        self.octave
    }
    /// returns a mutable reference to the current octave
    pub const fn octave_mut(&mut self) -> &mut Octave {
        &mut self.octave
    }
    /// set the pitch class of the note
    pub fn set_class(&mut self, class: Cls) -> &mut Self {
        self.class = class;
        self
    }
    /// set the octave of the note
    pub fn set_octave(&mut self, octave: Octave) -> &mut Self {
        self.octave = octave;
        self
    }
    /// consumes the current instance to create another with the given pitch class
    pub fn with_class<Pc: PitchClassRepr>(self) -> NoteBase<T, Pc> {
        NoteBase {
            class: Pc::new(),
            octave: self.octave,
            pitch: self.pitch,
        }
    }
    /// consumes the current instance to create another with the given octave
    pub fn with_octave(self, octave: Octave) -> Self {
        Self { octave, ..self }
    }
}

impl<C, T> core::fmt::Display for NoteBase<T, C>
where
    C: PitchClassRepr + core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}.{}", self.class, self.octave)
    }
}
