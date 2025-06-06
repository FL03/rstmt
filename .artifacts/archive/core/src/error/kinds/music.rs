/*
    Appellation: music <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    strum::AsRefStr,
    strum::EnumIs,
    strum::VariantNames,
    thiserror::Error,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "PascalCase")
)]
#[strum(serialize_all = "PascalCase")]
pub enum MusicErr {
    #[error("Invalid Interval")]
    InvalidInterval,
    #[error("Invalid Pitch: {0}")]
    InvalidPitch(String),
    #[error("Parse Error: {0}")]
    ParseError(String),
}

unsafe impl Send for MusicErr {}

unsafe impl Sync for MusicErr {}
