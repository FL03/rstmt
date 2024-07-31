/*
    Appellation: pitches <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{PitchClass, PitchTy};
use crate::error::{Error, MusicalError};

pitch_class! {
    #[default(C)]
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

pitch_class! {
    #[default(C)]
    pub enum Sharp {
        C = 1,
        D = 3,
        F = 6,
        G = 8,
        A = 10,
    }
}

pitch_class! {
    #[default(D)]
    pub enum Flat {
        D = 1,
        E = 3,
        G = 6,
        A = 8,
        B = 10,
    }
}

#[derive(
    Clone,
    Copy,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    smart_default::SmartDefault,
    strum::AsRefStr,
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
    pub fn try_from_value(value: PitchTy) -> Result<Self, Error<MusicalError>> {
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

impl core::fmt::Debug for Pitches {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use Pitches::*;
        match self {
            Flat(cls) => write!(f, "{}♭", cls.as_ref()),
            Natural(cls) => write!(f, "{}", cls.as_ref()),
            Sharp(cls) => write!(f, "{}#", cls.as_ref()),
        }
    }
}

impl core::fmt::Display for Pitches {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use Pitches::*;
        match self {
            Flat(cls) => write!(f, "{}♭", cls.as_ref()),
            Natural(cls) => write!(f, "{}", cls.as_ref()),
            Sharp(cls) => write!(f, "{}#", cls.as_ref()),
        }
    }
}

impl PitchClass for Pitches {
    seal!();

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
        Self::try_from_value(value).unwrap_or_default()
    }
}
