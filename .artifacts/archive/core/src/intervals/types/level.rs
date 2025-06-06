/*
    Appellation: level <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// [IntervalLevel] enumerates the possible levels of an interval
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::EnumIter,
    strum::EnumString,
    strum::VariantNames,
    strum::VariantArray,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[strum(serialize_all = "lowercase")]
pub enum IntervalLevel {
    #[default]
    Semitone = 1,
    Tone = 2,
    Third = 4,
    Fourth = 5,
    Fifth = 7,
    Sixth,
    Seventh = 11,
    Octave = 12,
}

impl IntervalLevel {
    pub const MOD: i8 = 12;

    pub fn from_i8(value: i8) -> Self {
        use IntervalLevel::*;
        match value.abs() % 12 {
            1 => Semitone,
            2 => Tone,
            3..=4 => Third,
            5 => Fourth,
            6..=8 => Fifth,
            9..=11 => Seventh,
            0 => Octave,
            _ => unreachable!(),
        }
    }
    /// A functional constructor for the [IntervalLevel::Semitone] variant.
    pub fn semitone() -> Self {
        IntervalLevel::Semitone
    }
    /// A functional constructor for the [IntervalLevel::Tone] variant.
    pub fn tone() -> Self {
        IntervalLevel::Tone
    }
    /// A functional constructor for the [IntervalLevel::Third] variant.
    pub fn third() -> Self {
        IntervalLevel::Third
    }
    /// A functional constructor for the [IntervalLevel::Fourth] variant.
    pub fn fourth() -> Self {
        IntervalLevel::Fourth
    }
    /// A functional constructor for the [IntervalLevel::Fifth] variant.
    pub fn fifth() -> Self {
        IntervalLevel::Fifth
    }
    /// A functional constructor for the [IntervalLevel::Sixth] variant.
    pub fn sixth() -> Self {
        IntervalLevel::Sixth
    }
    /// A functional constructor for the [IntervalLevel::Seventh] variant.
    pub fn seventh() -> Self {
        IntervalLevel::Seventh
    }
    /// A functional constructor for the [IntervalLevel::Octave] variant.
    pub fn octave() -> Self {
        IntervalLevel::Octave
    }
}
