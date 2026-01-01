/*
    Appellation: impl_note_ext <module>
    Created At: 2025.12.31:18:23:09
    Contrib: @FL03
*/
use crate::note::NoteBase;
use crate::pitch::{Accidental, RawPitchClass};

impl<P, K> core::fmt::Debug for NoteBase<P, K>
where
    P: RawPitchClass<Tag = K>,
    K: Accidental,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.aspn().as_str())
    }
}

impl<P, K> core::fmt::Display for NoteBase<P, K>
where
    P: RawPitchClass<Tag = K>,
    K: Accidental,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.aspn().as_str())
    }
}

impl<P, K> NoteBase<P, K>
where
    P: RawPitchClass<Tag = K>,
    K: Accidental,
{
    /// returns string formatted following the American Standard Pitch Notation (ASPN) of:
    /// "C.4", "D#.5", etc.
    pub fn aspn(&self) -> String {
        format!("{}.{}", self.class().name(), self.octave().value())
    }
}
