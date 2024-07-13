/*
    Appellation: pitches <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{PitchClass, PitchTy};
use crate::Octave;

pitch! {
    pub enum Natural {
        C = 0,
        D = 2,
        E = 4,
        F = 5,
        G = 7,
        A = 9,
        B = 11,
    }
}

pitch! {
    pub enum Sharp {
        C = 1,
        D = 3,
        F = 6,
        G = 8,
        A = 10,
    }
}

pitch! {
    pub enum Flat {
        D = 1,
        E = 3,
        G = 6,
        A = 8,
        B = 10,
    }
}

pub struct A {
    pub(crate) octave: Octave,
    pub(crate) pitch: PitchTy,
}

pub trait SharpPitch {
    private!();
}

pub trait FlatPitch {
    private!();
}

impl FlatPitch for A {
    seal!();
}

impl FlatPitch for Flat {
    seal!();
}

impl SharpPitch for Sharp {
    seal!();
}

pub trait AccidentalPitch: PitchClass {
    private!();
}

impl AccidentalPitch for Sharp {
    seal!();
}

impl AccidentalPitch for Flat {
    seal!();
}

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    smart_default::SmartDefault,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[repr(i8)]
#[strum(serialize_all = "lowercase")]
pub enum Pitches {
    Flat(Flat),
    #[default]
    Natural(Natural),
    Sharp(Sharp),
}

impl Pitches {
    pub fn try_from_value(value: PitchTy) -> crate::Result<Self> {
        if let Ok(n) = Natural::try_from_value(value) {
            Ok(n.as_class())
        } else if let Ok(s) = Sharp::try_from_value(value) {
            Ok(s.as_class())
        } else if let Ok(f) = Flat::try_from_value(value) {
            Ok(f.as_class())
        } else {
            Err(crate::Error::invalid_pitch("Invalid pitch value."))
        }
    }

    pub fn pitch(&self) -> PitchTy {
        match self {
            Pitches::Flat(f) => f.pitch(),
            Pitches::Natural(n) => n.pitch(),
            Pitches::Sharp(s) => s.pitch(),
        }
    }
}

impl PitchClass for Pitches {
    fn pitch(&self) -> PitchTy {
        self.pitch()
    }
}

impl From<Pitches> for PitchTy {
    fn from(pitch: Pitches) -> PitchTy {
        pitch.pitch()
    }
}

impl From<PitchTy> for Pitches {
    fn from(value: PitchTy) -> Pitches {
        Self::try_from_value(value).unwrap()
    }
}

mod impl_kinds {
    use super::*;

    impl Default for Natural {
        fn default() -> Self {
            Natural::C
        }
    }

    impl Default for Sharp {
        fn default() -> Self {
            Sharp::C
        }
    }

    impl Default for Flat {
        fn default() -> Self {
            Flat::D
        }
    }
}
