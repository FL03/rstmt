/*
    appellation: flags <module>
    authors: @FL03
*/

/// [`Accidental`] enumerates the two alternative states a note make take, either sharp or flat
/// and their respective states, single or double.
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
    strum::EnumCount,
    strum::EnumIs,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[strum(serialize_all = "lowercase")]
pub enum Accidentals {
    Sharp(crate::pitch::Sharp),
    Flat(crate::pitch::Flat),
    #[default]
    Natural(crate::pitch::Natural),
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
    strum::EnumCount,
    strum::EnumDiscriminants,
    strum::EnumIs,
)]
#[strum_discriminants(
    name(AccidentalFlag),
    derive(
        Hash,
        Ord,
        PartialOrd,
        strum::AsRefStr,
        strum::Display,
        strum::EnumCount,
        strum::EnumIs,
        strum::EnumIter,
        strum::EnumString,
        strum::VariantArray,
        strum::VariantNames,
    ),
    strum(serialize_all = "lowercase")
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase"),
    strum_discriminants(
        derive(serde::Deserialize, serde::Serialize),
        serde(rename_all = "lowercase")
    )
)]
pub enum AccidentalState<T> {
    /// A [`Single`](AccidentalState::Single) accidental state represents typical sharp/flat note
    Single(T),
    /// A [`Double`](AccidentalState::Double) accidental state represents a double sharp/flat note
    Double(T),
}
