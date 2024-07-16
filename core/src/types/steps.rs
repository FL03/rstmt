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
#[repr(i8)]
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
