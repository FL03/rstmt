/*
    Appellation: impl_note_ext <module>
    Created At: 2025.12.31:18:23:09
    Contrib: @FL03
*/
use crate::notes::note_base::NoteBase;
use crate::octave::Octave;
use crate::pitch::{Accidental, PitchClass, PitchClassRepr, RawPitchClass};
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

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

impl<P, K> PartialEq<str> for NoteBase<P, K>
where
    P: RawPitchClass<Tag = K>,
    K: Accidental,
{
    fn eq(&self, other: &str) -> bool {
        self.aspn() == other
    }
}

impl<P, K> PartialEq<&str> for NoteBase<P, K>
where
    P: RawPitchClass<Tag = K>,
    K: Accidental,
{
    fn eq(&self, other: &&str) -> bool {
        self.aspn() == *other
    }
}

#[cfg(feature = "alloc")]
impl<P, K> core::str::FromStr for NoteBase<P, K>
where
    P: PitchClassRepr<Tag = K>,
    K: Accidental + core::str::FromStr<Err = crate::Error>,
{
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 2 {
            return Err(crate::Error::FromStrParseError);
        }
        let lex_class = parts[0];
        let lex_octave = parts[1];
        let class = lex_class.parse::<PitchClass<P, K>>()?;
        let octave = lex_octave.parse::<isize>().expect("Failed to parse octave");
        Ok(Self::new(class, Octave(octave)))
    }
}
