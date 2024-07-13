/*
    Appellation: note <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::pitch::{PitchT, PitchTy};
use crate::Octave;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Note<P = PitchTy> {
    pub(crate) octave: Octave,
    pub(crate) pitch: P,
}

impl<P> Note<P>
where
    P: PitchT,
{
    pub fn new(octave: Octave, pitch: P) -> Self {
        Self { octave, pitch }
    }
    /// Returns an owned instance of the note's octave
    pub const fn octave(&self) -> &Octave {
        &self.octave
    }

    pub const fn pitch(&self) -> &P {
        &self.pitch
    }

    pub fn set_octave(&mut self, octave: Octave) {
        self.octave = octave;
    }

    pub fn with_octave(self, octave: Octave) -> Self {
        Self { octave, ..self }
    }

    pub fn with_pitch<P2>(self, pitch: P2) -> Note<P2>
    where
        P2: PitchT,
    {
        Note {
            octave: self.octave,
            pitch,
        }
    }
}

impl<P> core::fmt::Display for Note<P>
where
    P: PitchT,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}.{}", self.pitch, self.octave)
    }
}
