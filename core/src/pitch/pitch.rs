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
pub struct Pitch(pub PitchTy);

impl Pitch {
    pub const MOD: PitchTy = crate::MODULUS;

    pub fn new(pitch: PitchTy) -> Self {
        Self(pitch)
    }
    /// Consumes the object, returning the assigned pitch value.
    pub fn into_inner(self) -> PitchTy {
        self.0
    }
    /// A functional accessor for the pitch value.
    pub const fn get(&self) -> &PitchTy {
        &self.0
    }
    /// returns a mutable reference to the inner value of the pitch;
    pub fn get_mut(&mut self) -> &mut PitchTy {
        &mut self.0
    }
    /// A functional mutator for the pitch value.
    pub fn set(&mut self, val: PitchTy) {
        self.0 = val;
    }
}

impl Pitch {
    /// Returns a new instance of the class representing the given pitch.
    pub fn class(&self) -> Pitches {
        Pitches::try_from_value(self.0.pymod(Self::MOD).abs()).unwrap()
    }
    /// Consumes the pitch; returns a new instance of the class representing the given pitch.
    pub fn into_class(self) -> Pitches {
        self.class()
    }
    ///
    pub fn map<F>(self, f: F) -> Self
    where
        F: Fn(PitchTy) -> PitchTy,
    {
        Self(f(self.0))
    }

    pub fn map_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut PitchTy),
    {
        f(&mut self.0)
    }
    /// Consumes the current instance and applies the given function to the pitch value, returning
    /// a new instance of [Pitch].
    pub fn map_once<F>(self, f: F) -> Self
    where
        F: FnOnce(PitchTy) -> PitchTy,
    {
        Self(f(self.0))
    }
    /// Consumes the current instance and computes the absolute value of the pitch, returning
    /// a new instance of [Pitch].
    pub fn abs(self) -> Self {
        self.map(|p| p.abs())
    }
    /// Returns the absolute value of the remainder of the pitch divided by the modulus.
    pub fn absmod(self) -> Self {
        self.map(|p| p.pymod(Self::MOD).abs())
    }
    /// The [`octmod`](Pitch::octmod) method computes the modulus of the current pitch using
    /// Python's modulo operator, `%`. This method is useful for pitch arithmetic due to its
    /// unique sign-handling behavior.
    pub fn pymod(self) -> Self {
        self.map(|p| p.pymod(Self::MOD))
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

impl core::borrow::Borrow<PitchTy> for Pitch {
    fn borrow(&self) -> &PitchTy {
        &self.0
    }
}

impl core::borrow::BorrowMut<PitchTy> for Pitch {
    fn borrow_mut(&mut self) -> &mut PitchTy {
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
        Self(pitch.value())
    }
}

impl From<Pitch> for Pitches {
    fn from(pitch: Pitch) -> Self {
        pitch.class()
    }
}
