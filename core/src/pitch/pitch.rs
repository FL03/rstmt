/*
    Appellation: pitch <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{PitchTy, Pitches};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Pitch(pub PitchTy);

impl Pitch {
    pub fn new(pitch: PitchTy) -> Self {
        Self(pitch)
    }
    /// Returns a new instance of the class representing the given pitch.
    pub fn as_class(&self) -> Pitches {
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

impl core::fmt::Display for Pitch {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}
