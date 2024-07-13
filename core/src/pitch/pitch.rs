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

impl core::fmt::Display for Pitch {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

macro_rules! impl_ops {
    (@impl $name:ident impls $trait:ident.$call:ident$(<$T:ident>)?$(=> $out:ty)? $(where $($rest:tt)*)?) => {
        impl_ops!(@impl $name impls $trait.$call($name)$(<$T>)? $(=> $out)? $(where $($rest)*)?);
    };
    (@impl $name:ident impls $trait:ident.$call:ident($rhs:ident)$(<$T:ident>)? $(where $($rest:tt)*)?) => {
        impl_ops!(@impl $name impls $trait.$call($name)$(<$T>)? => Self $(where $($rest)*)?);
    };
    (@impl $name:ident impls $trait:ident.$call:ident($rhs:ident)$(<$T:ident>)? => $out:ty $(where $($rest:tt)*)?) => {
        impl$(<$T>)? core::ops::$trait<$rhs> for $name $(where $($rest)*)? {
            type Output = $out;

            fn $call(self, rhs: $rhs) -> Self::Output {
                ::core::ops::$trait::$call(self.0, rhs.0)
            }
        }
    };
    ($name:ident impls $($rest:tt)*) => {
        impl_ops!(@impl $name impls $($rest)*);
    };
    (numops: $($name:ident$(<$T:ident>)? $(=> $out:ty)?),* $(,)?) => {
        $(
            impl_ops!(@impl $name impls Add.add$(<$T>)? $(=> $out)?);
            impl_ops!(@impl $name impls Div.div$(<$T>)? $(=> $out)?);
            impl_ops!(@impl $name impls Mul.mul$(<$T>)? $(=> $out)?);
            impl_ops!(@impl $name impls Rem.rem$(<$T>)? $(=> $out)?);
            impl_ops!(@impl $name impls Sub.sub$(<$T>)? $(=> $out)?);
        )*
    };
}

impl_ops!(numops: Pitch => i8);
