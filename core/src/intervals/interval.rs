/*
    Appellation: interval <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::IntervalTy;
use crate::Step;

/// The number of an interval.
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
#[repr(i8)]
#[strum(serialize_all = "lowercase")]
pub enum IntervalLevel {
    #[default]
    Unison = 1,
    Second = 2,
    Third,
    Fourth = 5,
    Fifth,
    Sixth,
    Seventh,
    Octave = 12,
}

#[allow(dead_code)]
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Interval<T = crate::Note> {
    pub(crate) distance: IntervalTy,
    pub(crate) level: IntervalLevel,
    pub(crate) source: T,
    pub(crate) step: Step,
}

impl<T> Interval<T> {}
