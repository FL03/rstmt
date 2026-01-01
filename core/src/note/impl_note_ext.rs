/*
    Appellation: impl_note_ext <module>
    Created At: 2025.12.31:18:23:09
    Contrib: @FL03
*/
use crate::note::NoteBase;
use crate::pitch::{Accidental, PitchClassRepr, RawPitchClass};

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

impl<P, K> core::str::FromStr for NoteBase<P, K>
where
    P: PitchClassRepr<Tag = K>,
    K: Accidental,
    <P as core::str::FromStr>::Err: core::fmt::Debug,
{
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid ASPN format: {}", s).into());
        }
        let class_str = parts[0];
        let octave_str = parts[1];
        let _symbol = class_str
            .parse::<P>()
            .map_err(|e| anyhow::anyhow!("Failed to parse pitch class: {:?}", e))?;
        let octave = octave_str
            .parse::<isize>()
            .map_err(|e| anyhow::anyhow!("Failed to parse octave: {:?}", e))?;
        Ok(Self::from_octave(crate::octave::Octave(octave)))
    }
}
