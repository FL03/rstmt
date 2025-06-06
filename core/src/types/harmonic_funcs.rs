/*
    Appellation: harmonic_functions <module>
    Contrib: @FL03
*/

/// Harmonic functions in tonal music
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
    strum::EnumIter,
    strum::EnumString,
    strum::VariantArray,
    strum::VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Deserialize, serde_derive::Serialize),
    serde(rename_all = "snake_case")
)]
#[strum(serialize_all = "snake_case")]
pub enum HarmonicFunction {
    Ambiguous,
    Dominant,
    LeadingTone,
    Mediant,
    Predominant,
    Secondary,
    Subdominant,
    Submediant,
    Supertonic,
    Tonic,
}
