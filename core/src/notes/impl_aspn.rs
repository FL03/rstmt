/*
    Appellation: impl_aspn <module>
    Created At: 2025.12.20:08:16:03
    Contrib: @FL03
*/
use super::Aspn;
use crate::octave::Octave;
use rstmt_traits::PitchMod;

impl Aspn {
    pub fn new(class: usize, Octave(octave): Octave) -> Self {
        Self {
            class,
            octave: Octave(octave),
        }
    }
    /// returns a new note from a pitch value
    pub fn from_pitch(pitch: usize) -> Self {
        Self::new(pitch.pmod(), Octave(4))
    }
    /// returns a copy to the index of the note's class
    pub const fn class(&self) -> usize {
        self.class
    }
    /// returns a mutable reference to the index of the note's class
    pub fn class_mut(&mut self) -> &mut usize {
        &mut self.class
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
    pub fn set_class(&mut self, class: usize) -> &mut Self {
        self.class = class.pmod();
        self
    }
    /// set the octave of the note
    pub fn set_octave(&mut self, octave: Octave) -> &mut Self {
        self.octave = octave;
        self
    }
    /// consumes the current instance to create another with the given pitch class
    pub fn with_class(self, class: usize) -> Self {
        Self { class, ..self }
    }
    /// consumes the current instance to create another with the given octave
    pub fn with_octave(self, octave: Octave) -> Self {
        Self { octave, ..self }
    }
}
