/*
    Appellation: intervals <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
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
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub enum Intervals {
    Semitone = 1,
    Tone = 2,
    Thirds(Third),
    Fourths(Fourth),
    Fifths(Fifth),
    Sevenths(Seventh),
}

impl Intervals {
    pub fn from_semitone() -> Self {
        Intervals::Semitone
    }

    pub fn from_tone() -> Self {
        Intervals::Tone
    }

    pub fn from_thirds(third: Third) -> Self {
        Intervals::Thirds(third)
    }

    pub fn from_fourths(fourth: Fourth) -> Self {
        Intervals::Fourths(fourth)
    }

    pub fn from_fifths(fifth: Fifth) -> Self {
        Intervals::Fifths(fifth)
    }

    pub fn from_sevenths(seventh: Seventh) -> Self {
        Intervals::Sevenths(seventh)
    }

    pub fn name(&self) -> &str {
        self.as_ref()
    }

    pub fn value(&self) -> i8 {
        match *self {
            Intervals::Semitone => 1,
            Intervals::Tone => 2,
            Intervals::Thirds(third) => third as i8,
            Intervals::Fourths(fourth) => fourth as i8,
            Intervals::Fifths(fifth) => fifth as i8,
            Intervals::Sevenths(seventh) => seventh as i8,
        }
    }
}



impl From<Third> for Intervals {
    fn from(third: Third) -> Self {
        Intervals::Thirds(third)
    }
}

impl From<Fourth> for Intervals {
    fn from(fourth: Fourth) -> Self {
        Intervals::Fourths(fourth)
    }
}

impl From<Fifth> for Intervals {
    fn from(fifth: Fifth) -> Self {
        Intervals::Fifths(fifth)
    }
}

impl From<Seventh> for Intervals {
    fn from(seventh: Seventh) -> Self {
        Intervals::Sevenths(seventh)
    }
}


interval! {
    pub enum Third {
        Minor = 3,
        Major = 4,
    }
}

interval! {
    pub enum Fourth {
        Perfect = 5,
    }
}

interval! {
    pub enum Fifth {
        Augmented = 8,
        Perfect = 7,
        Diminished = 6,
    }
}


interval! {
    pub enum Seventh {
        Augmented = 12,
        Diminished = 9,
        Major = 11,
        Minor = 10,
    }
}
