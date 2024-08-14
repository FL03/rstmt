/*
    Appellation: pitches <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{PitchClass, PitchTy};

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
    pub fn from_flat(note: Flat) -> Self {
        Self::Flat(note)
    }

    pub fn from_natural(note: Natural) -> Self {
        Self::Natural(note)
    }

    pub fn from_sharp(note: Sharp) -> Self {
        Self::Sharp(note)
    }

    pub fn a() -> Self {
        Self::Natural(Natural::A)
    }

    pub fn a_flat() -> Self {
        Self::Flat(Flat::A)
    }

    pub fn a_sharp() -> Self {
        Self::Sharp(Sharp::A)
    }

    pub fn b() -> Self {
        Self::Natural(Natural::B)
    }

    pub fn b_flat() -> Self {
        Self::Flat(Flat::B)
    }

    pub fn c() -> Self {
        Self::Natural(Natural::C)
    }

    pub fn c_sharp() -> Self {
        Self::Sharp(Sharp::C)
    }

    pub fn d() -> Self {
        Self::Natural(Natural::D)
    }

    pub fn d_flat() -> Self {
        Self::Flat(Flat::D)
    }

    pub fn d_sharp() -> Self {
        Self::Sharp(Sharp::D)
    }

    pub fn e() -> Self {
        Self::Natural(Natural::E)
    }

    pub fn e_flat() -> Self {
        Self::Flat(Flat::E)
    }

    pub fn f() -> Self {
        Self::Natural(Natural::F)
    }

    pub fn f_sharp() -> Self {
        Self::Sharp(Sharp::F)
    }

    pub fn g() -> Self {
        Self::Natural(Natural::G)
    }

    pub fn g_flat() -> Self {
        Self::Flat(Flat::G)
    }

    pub fn g_sharp() -> Self {
        Self::Sharp(Sharp::G)
    }
    // TODO: Find a way to correctly resolve notes that share the same pitch values; e.g. D♯ == E♭
    pub fn try_from_value(value: PitchTy) -> Result<Self, crate::Error> {
        if let Ok(note) = Natural::try_from_value(value) {
            Ok(note.as_class())
        } else if let Ok(note) = Flat::try_from_value(value) {
            Ok(note.as_class())
        } else if let Ok(note) = Sharp::try_from_value(value) {
            Ok(note.as_class())
        } else {
            Err(crate::Error::music_error("Invalid pitch value."))
        }
    }

    pub fn value(&self) -> PitchTy {
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

impl core::cmp::PartialEq for Pitches {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl PitchClass for Pitches {
    seal!();

    fn pitch(&self) -> PitchTy {
        self.value()
    }
}

impl From<Pitches> for PitchTy {
    fn from(pitch: Pitches) -> PitchTy {
        pitch.value()
    }
}

impl From<PitchTy> for Pitches {
    fn from(value: PitchTy) -> Pitches {
        Self::try_from_value(value).unwrap_or_default()
    }
}
