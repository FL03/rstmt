/*
    Appellation: music <module>
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
    strum::EnumIs,
    strum::EnumString,
    strum::VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "PascalCase")
)]
#[strum(serialize_all = "PascalCase")]
pub enum MusicalError {
    InvalidInterval,
    InvalidPitch,
}

#[cfg(feature = "std")]
impl std::error::Error for MusicalError {}

unsafe impl Send for MusicalError {}

unsafe impl Sync for MusicalError {}
