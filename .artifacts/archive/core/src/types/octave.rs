/*
    Appellation: octave <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
/// The default octave value.
pub(crate) const DEFAULT_OCTAVE: OctaveTy = 4;
/// A type alias indiciating the wrapped type within an [octave](crate::Octave).
pub(crate) type OctaveTy = i8;

/// An octave describes the interval between one musical pitch and another with either half or double its frequency.
/// Any changes made to a notes octave simply translates the same pitch class to another register.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Octave(pub OctaveTy);

impl Octave {
    pub fn new(Octave(octave): Octave) -> Self {
        Self(octave)
    }

    pub fn from_value(value: OctaveTy) -> Self {
        Self(value)
    }
    /// Returns an immutable reference to the wrapped value.
    pub const fn get(&self) -> &OctaveTy {
        &self.0
    }
    /// Returns a mutable reference to the wrapped value.
    pub fn get_mut(&mut self) -> &mut OctaveTy {
        &mut self.0
    }
    /// Sets the octave value.
    pub fn set(&mut self, octave: OctaveTy) {
        self.0 = octave;
    }
    /// Consumes the octave and returns the wrapped value.
    pub fn into_inner(self) -> OctaveTy {
        self.0
    }

    pub fn value(&self) -> OctaveTy {
        self.0
    }
}

impl AsRef<OctaveTy> for Octave {
    fn as_ref(&self) -> &OctaveTy {
        &self.0
    }
}

impl AsMut<OctaveTy> for Octave {
    fn as_mut(&mut self) -> &mut OctaveTy {
        &mut self.0
    }
}

impl core::borrow::Borrow<OctaveTy> for Octave {
    fn borrow(&self) -> &OctaveTy {
        &self.0
    }
}

impl core::borrow::BorrowMut<OctaveTy> for Octave {
    fn borrow_mut(&mut self) -> &mut OctaveTy {
        &mut self.0
    }
}

impl core::ops::Deref for Octave {
    type Target = OctaveTy;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for Octave {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl core::fmt::Binary for Octave {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Binary::fmt(&self.0, f)
    }
}

impl core::fmt::Display for Octave {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl core::fmt::LowerExp for Octave {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::LowerExp::fmt(&self.0, f)
    }
}

impl core::fmt::LowerHex for Octave {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::LowerHex::fmt(&self.0, f)
    }
}

impl core::fmt::Octal for Octave {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Octal::fmt(&self.0, f)
    }
}

impl core::fmt::UpperExp for Octave {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::UpperExp::fmt(&self.0, f)
    }
}

impl Default for Octave {
    fn default() -> Self {
        Self(DEFAULT_OCTAVE)
    }
}

wrapper_ops!(Octave::<i8>: Add.add, Div.div, Mul.mul, Rem.rem, Sub.sub);

wrapper_unop!(Octave impls Neg.neg, Not.not);

impl num::One for Octave {
    fn one() -> Self {
        Self(OctaveTy::one())
    }
}

impl num::Zero for Octave {
    fn zero() -> Self {
        Self(OctaveTy::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl num::Num for Octave {
    type FromStrRadixErr = <OctaveTy as num::Num>::FromStrRadixErr;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        OctaveTy::from_str_radix(s, radix).map(Self)
    }
}
