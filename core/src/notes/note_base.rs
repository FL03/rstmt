/*
    Appellation: note <module>
    Contrib: @FL03
*/
use super::Octave;
use crate::freq::RawFrequency;
use crate::pitch::{self, Pitch, PitchClass};


/// The [`NoteBase`] is a generic representation of a musical note
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields, default, rename_all = "snake_case")
)]
#[repr(C)]
pub struct NoteBase<T, Cls = pitch::C>
where
    T: RawFrequency,
{
    pub(crate) class: Cls,
    pub(crate) octave: Octave,
    pub(crate) pitch: Pitch<T>,
}

impl<T, Cls> NoteBase<T, Cls>
where
    T: RawFrequency,
    Cls: PitchClass,
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
    pub fn with_class<Pc: PitchClass>(self) -> NoteBase<T, Pc> {
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
    C: PitchClass,
    T: RawFrequency,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}.{}", self.class, self.octave)
    }
}

impl<T, C> Default for NoteBase<T, C>
where
    T: RawFrequency + Default,
    C: PitchClass,
{
    fn default() -> Self {
        Self {
            class: C::new(),
            octave: Octave::default(),
            pitch: Pitch::default(),
        }
    }
}
