/*
    Appellation: steps <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// A step between notes.
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
    strum::VariantArray,
    strum::VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[non_exhaustive]
#[repr(usize)]
#[strum(serialize_all = "lowercase")]
pub enum Step {
    #[default]
    /// A semitone step.
    Half = 1,
    /// A tone step.
    Whole = 2,
    /// A tritone step.
    Tritone = 3,
}

impl Step {
    /// A functional constructor for [Step::Half].
    pub fn half() -> Self {
        Self::Half
    }
    /// A functional constructor for [Step::Whole].
    pub fn whole() -> Self {
        Self::Whole
    }
    /// A functional constructor for [Step::Tritone].
    pub fn tritone() -> Self {
        Self::Tritone
    }
    /// Converts an [i8] value into a [Step] by taking the modulus of the value.
    /// The function uses a modulator of 3 to determine the step since there are
    /// only three actionable steps ([half](Step::Half), [whole](Step::Whole), and [tritone](Step::Tritone)).
    pub fn from_usize(value: impl Into<usize>) -> Self {
        match value.into() % 3 {
            1 => Self::Half,
            2 => Self::Whole,
            _ => Self::Tritone,
        }
    }
}
