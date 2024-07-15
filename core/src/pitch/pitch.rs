/*
    Appellation: pitch <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{PitchTy, Pitches};
use crate::PitchMod;

/// A [pitch](Pitch) is a discrete tone with an individual frequency that may be
/// classified as a [pitch class](Pitches).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(transparent)]
pub struct Pitch(pub(crate) PitchTy);

impl Pitch {
    pub fn new(pitch: impl Into<PitchTy>) -> Self {
        Self(pitch.into().pitchmod())
    }
    /// Returns the absolute value of the remainder of the pitch divided by the modulus.
    pub fn abs(&self) -> Self {
        self.pitchmod()
    }
    /// Returns a new instance of the class representing the given pitch.
    pub fn class(&self) -> Pitches {
        Pitches::try_from_value(self.0).unwrap()
    }
    /// Consumes the pitch; returns a new instance of the class representing the given pitch.
    pub fn into_class(self) -> Pitches {
        Pitches::try_from_value(self.0).unwrap()
    }
    /// Consumes the object, returning the assigned pitch value.
    pub fn into_inner(self) -> PitchTy {
        self.0
    }
    /// A functional accessor for the pitch value.
    pub fn value(&self) -> PitchTy {
        self.0
    }
}

impl AsRef<PitchTy> for Pitch {
    fn as_ref(&self) -> &PitchTy {
        &self.0
    }
}

impl AsMut<PitchTy> for Pitch {
    fn as_mut(&mut self) -> &mut PitchTy {
        &mut self.0
    }
}

impl Default for Pitch {
    fn default() -> Self {
        Self(super::Natural::default().pitch())
    }
}

impl core::ops::Deref for Pitch {
    type Target = PitchTy;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl core::ops::DerefMut for Pitch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl core::fmt::Binary for Pitch {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Binary::fmt(&self.0, f)
    }
}

impl core::fmt::Display for Pitch {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl core::fmt::LowerExp for Pitch {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::LowerExp::fmt(&self.0, f)
    }
}

impl core::fmt::LowerHex for Pitch {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::LowerHex::fmt(&self.0, f)
    }
}

impl core::fmt::Octal for Pitch {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Octal::fmt(&self.0, f)
    }
}

impl core::fmt::UpperExp for Pitch {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::UpperExp::fmt(&self.0, f)
    }
}

impl From<PitchTy> for Pitch {
    fn from(pitch: PitchTy) -> Self {
        Self(pitch)
    }
}

impl From<Pitch> for PitchTy {
    fn from(pitch: Pitch) -> Self {
        pitch.0
    }
}

impl From<Pitches> for Pitch {
    fn from(pitch: Pitches) -> Self {
        Self(pitch.pitch())
    }
}

impl From<Pitch> for Pitches {
    fn from(pitch: Pitch) -> Self {
        Pitches::try_from_value(pitch.0).unwrap()
    }
}
