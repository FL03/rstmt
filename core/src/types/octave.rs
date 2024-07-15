/*
    Appellation: octave <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// A type alias for an `octave`; Musically speaking, an octave is the interval (distance) between one musical pitch and another
/// with either half or double its frequency.
pub(crate) type OctaveTy = i8;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Octave(pub(crate) OctaveTy);

impl Octave {
    pub fn new(octave: OctaveTy) -> Self {
        Self(octave)
    }

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

impl core::fmt::Display for Octave {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
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
