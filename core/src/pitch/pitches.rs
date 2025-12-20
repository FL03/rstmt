/*
    Appellation: pitches <module>
    Created At: 2025.12.20:10:02:21
    Contrib: @FL03
*/
use crate::freq::Frequency;

/// Musically, a pitch is defined to be a discrete frequency that may be symbolically
/// represented via a pitch class.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
#[repr(transparent)]
pub struct Pitch<T = f64>(pub T);

/// A discrete pitch with a class and frequency.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields, rename_all = "snake_case")
)]
#[repr(C)]
pub struct ClassifiedPitch<T = f32> {
    pub(crate) class: isize,
    pub(crate) freq: Frequency<T>,
}
