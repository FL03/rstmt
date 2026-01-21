/*
    Appellation: impl_note_repr <module>
    Created At: 2026.01.20:08:59:53
    Contrib: @FL03
*/
use crate::notes::note_base::NoteBase;
use crate::octave::Octave;
use crate::pitch::{Flat, Natural, PitchClass, RawPitchClass, Sharp};

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
