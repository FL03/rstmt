/*
    Appellation: pitch <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{PitchTy, Pitches};
use crate::PyMod;

/// A [pitch](Pitch) is a discrete tone with an individual frequency that may be
/// classified as a [pitch class](Pitches).
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(transparent)]
pub struct Pitch(pub(crate) PitchTy);

impl Pitch {
    const MOD: PitchTy = crate::MODULUS;

    pub fn new(pitch: PitchTy) -> Self {
        Self(pitch)
    }

    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }
    /// Returns the absolute value of the remainder of the pitch divided by the modulus.
    pub fn absmod(self) -> Self {
        Self(self.0.pymod(Self::MOD).abs())
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

impl core::fmt::Debug for Pitch {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_tuple(&self.class().to_string())
            .field(&self.0)
            .finish()
    }
}

impl core::fmt::Display for Pitch {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}({})", self.class(), self.0)
    }
}
macro_rules! impl_fmt {
    (@impl $trait:ident) => {
        impl ::core::fmt::$trait for Pitch {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                ::core::fmt::$trait::fmt(&self.0, f)
            }
        }
    };
    ($($trait:ident),* $(,)?) => {
        $(
            impl_fmt!(@impl $trait);
        )*
    };
}

impl_fmt!(Binary, LowerHex, Octal, UpperHex);

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
