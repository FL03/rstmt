/*
    Appellation: impl_note_base <module>
    Created At: 2025.12.20:09:35:09
    Contrib: @FL03
*/
use super::NoteBase;
use crate::octave::Octave;
use crate::pitch::{Accidental, Flat, Natural, PitchClass, RawPitchClass, Sharp};

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

impl<P> NoteBase<P>
where
    P: RawPitchClass,
{
    /// returns a new natural note of the given class and octave
    pub fn natural(class: P, octave: Octave) -> NoteBase<P, Natural>
    where
        P: RawPitchClass<Tag = Natural>,
    {
        NoteBase::new(PitchClass::natural(class), octave)
    }
    /// returns a new sharp note of the given class and octave
    pub fn sharp(class: P, octave: Octave) -> NoteBase<P, Sharp>
    where
        P: RawPitchClass<Tag = Sharp>,
    {
        NoteBase::new(PitchClass::sharp(class), octave)
    }
    /// returns a new flat note of the given class and octave
    pub fn flat(class: P, octave: Octave) -> NoteBase<P, Flat>
    where
        P: RawPitchClass<Tag = Flat>,
    {
        NoteBase::new(PitchClass::flat(class), octave)
    }
}
