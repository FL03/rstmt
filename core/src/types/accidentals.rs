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
    variants::VariantConstructors,
    strum::EnumCount,
    strum::EnumIs,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[strum(serialize_all = "lowercase")]
pub enum Accidental {
    Sharp(AccidentalState),
    Flat(AccidentalState),
}

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
    variants::VariantConstructors,
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
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[strum(serialize_all = "lowercase")]
pub enum AccidentalState {
    #[default]
    /// A [`Single`](AccidentalState::Single) accidental state represents typical sharp/flat note
    Single,
    /// A [`Double`](AccidentalState::Double) accidental state represents a double sharp/flat note
    Double,
}
