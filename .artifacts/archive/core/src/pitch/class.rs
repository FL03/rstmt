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
    PartialEq,
    PartialOrd,
    smart_default::SmartDefault,
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

macro_rules! note_constructor {
    (@impl $note:ident::$call:ident(flat)) => {
        make_variant!(@impl pub Natural::$call($note));
        paste::paste! {
            make_variant!(@impl pub Flat::[<$call _flat>]($note));
        }
    };
    (@impl $note:ident::$call:ident(sharp)) => {
        make_variant!(@impl pub Natural::$call($note));
        paste::paste! {
            make_variant!(@impl pub Sharp::[<$call _sharp>]($note));
        }
    };
    (@impl $note:ident::$call:ident) => {
        make_variant!(@impl pub Natural::$call($note));
        paste::paste! {
            make_variant!(@impl pub Flat::[<$call _flat>]($note));
            make_variant!(@impl pub Sharp::[<$call _sharp>]($note));
        }
    };

    ($($note:ident::$call:ident$(($ctx:ident))?),* $(,)?) => {
        $(
            note_constructor!(@impl $note::$call$(($ctx))?);
        )*
    };
}

impl Flat {
    pub const SYMBOL: char = '♭';
}

impl Sharp {
    pub const SYMBOL: char = '#';
}

impl Pitches {
    /// Construct a new pitch class from a flat note.
    pub fn from_flat(note: Flat) -> Self {
        Self::Flat(note)
    }
    /// Construct a new pitch class from a natural note.
    pub fn from_natural(note: Natural) -> Self {
        Self::Natural(note)
    }
    /// Construct a new pitch class from a sharp note.
    pub fn from_sharp(note: Sharp) -> Self {
        Self::Sharp(note)
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

    pub fn get(&self) -> PitchTy {
        match self {
            Pitches::Flat(f) => f.pitch(),
            Pitches::Natural(n) => n.pitch(),
            Pitches::Sharp(s) => s.pitch(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Pitches::Flat(note) => format!("{note}♭"),
            Pitches::Sharp(note) => format!("{note}#"),
            Pitches::Natural(note) => format!("{note}"),
        }
    }

    note_constructor!(
        A::a,
        B::b(flat),
        C::c(sharp),
        D::d,
        E::e(flat),
        F::f(sharp),
        G::g
    );
}

impl AsRef<str> for Pitches {
    fn as_ref(&self) -> &str {
        use Pitches::*;
        match self {
            Flat(cls) => cls.as_ref(),
            Natural(cls) => cls.as_ref(),
            Sharp(cls) => cls.as_ref(),
        }
    }
}

impl core::str::FromStr for Flat {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if dbg!(!s.contains("#")) {
            return Err(crate::Error::parse_error(
                "the note doesn't have the symbol '#'",
            ));
        };
        match s.to_uppercase().trim().trim_end_matches(Flat::SYMBOL) {
            "D" => Ok(Flat::D),
            "E" => Ok(Flat::E),
            "G" => Ok(Flat::G),
            "A" => Ok(Flat::A),
            "B" => Ok(Flat::B),
            _ => Err(crate::Error::parse_error("Invalid flat note.")),
        }
    }
}

impl core::str::FromStr for Sharp {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let note = dbg!(s.trim().to_uppercase());
        if dbg!(!s.contains("#")) {
            return Err(crate::Error::parse_error(
                "the note doesn't have the symbol '#'",
            ));
        };
        match note.trim_end_matches("#") {
            "C" => Ok(Sharp::C),
            "D" => Ok(Sharp::D),
            "F" => Ok(Sharp::F),
            "G" => Ok(Sharp::G),
            "A" => Ok(Sharp::A),
            _ => Err(crate::Error::parse_error("Invalid sharp note.")),
        }
    }
}

impl core::str::FromStr for Natural {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_uppercase();
        match s.as_str() {
            "C" => Ok(Natural::C),
            "D" => Ok(Natural::D),
            "E" => Ok(Natural::E),
            "F" => Ok(Natural::F),
            "G" => Ok(Natural::G),
            "A" => Ok(Natural::A),
            "B" => Ok(Natural::B),
            _ => Err(crate::Error::parse_error("Invalid natural note.")),
        }
    }
}
impl core::str::FromStr for Pitches {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("♭") {
            Flat::from_str(s).map(Pitches::from_flat)
        } else if s.contains("#") {
            Sharp::from_str(s).map(Pitches::from_sharp)
        } else {
            Natural::from_str(s).map(Pitches::from_natural)
        }
    }
}

impl core::fmt::Debug for Pitches {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl core::fmt::Display for Pitches {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl core::cmp::PartialEq<Natural> for Pitches {
    fn eq(&self, other: &Natural) -> bool {
        self.get() == other.get()
    }
}

impl core::cmp::PartialEq<Flat> for Pitches {
    fn eq(&self, other: &Flat) -> bool {
        self.get() == other.get()
    }
}

impl core::cmp::PartialEq<Sharp> for Pitches {
    fn eq(&self, other: &Sharp) -> bool {
        self.get() == other.get()
    }
}

impl From<Pitches> for PitchTy {
    fn from(pitch: Pitches) -> PitchTy {
        pitch.get()
    }
}

impl From<PitchTy> for Pitches {
    fn from(value: PitchTy) -> Pitches {
        Self::try_from_value(value).unwrap_or_default()
    }
}
