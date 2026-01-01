/*
    Appellation: impl_note_ext <module>
    Created At: 2025.12.31:18:23:09
    Contrib: @FL03
*/
use crate::note::NoteBase;
use crate::octave::Octave;
use crate::pitch::{Accidental, PitchClass, PitchClassRepr, RawAccidental, RawPitchClass};

impl<P, K> core::fmt::Debug for NoteBase<P, K>
where
    P: RawPitchClass<Tag = K>,
    K: RawAccidental,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.aspn().as_str())
    }
}

impl<P, K> core::fmt::Display for NoteBase<P, K>
where
    P: RawPitchClass<Tag = K>,
    K: RawAccidental,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.aspn().as_str())
    }
}

impl<P, K> PartialEq<str> for NoteBase<P, K>
where
    P: RawPitchClass<Tag = K>,
    K: RawAccidental,
{
    fn eq(&self, other: &str) -> bool {
        self.aspn() == other
    }
}

impl<P, K> PartialEq<&str> for NoteBase<P, K>
where
    P: RawPitchClass<Tag = K>,
    K: RawAccidental,
{
    fn eq(&self, other: &&str) -> bool {
        self.aspn() == *other
    }
}
#[cfg(feature = "alloc")]
impl<P, K> core::str::FromStr for NoteBase<P, K>
where
    P: PitchClassRepr<Tag = K>,
    K: Accidental,
    <P as core::str::FromStr>::Err: core::fmt::Debug,
{
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: alloc::vec::Vec<&str> = s.split('.').collect();
        if parts.len() != 2 {
            return Err(crate::error::Error::FromStrParseError);
        }
        let class_str = parts[0];
        let octave_str = parts[1];
        let class = class_str
            .parse::<PitchClass<P, K>>()
            .expect("Failed to parse pitch class");
        let octave = octave_str.parse::<isize>().expect("Failed to parse octave");
        Ok(Self::new(class, Octave(octave)))
    }
}
