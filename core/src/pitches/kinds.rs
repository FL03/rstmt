/*
    Appellation: pitches <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{PitchClass, PitchTy};

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
        A = 10,
        C = 1,
        D = 3,
        F = 6,
        G = 8,
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
    pub fn from_value(value: PitchTy) -> crate::Result<Self> {
        if let Ok(n) = Natural::new(value) {
            Ok(n.as_class())
        } else if let Ok(s) = Sharp::new(value) {
            Ok(s.as_class())
        } else if let Ok(f) = Flat::new(value) {
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
        Self::from_value(value).unwrap()
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
