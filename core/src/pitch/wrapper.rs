/*
    Appellation: pitch <module>
    Contrib: @FL03
*/
mod impl_pitch;
mod impl_pitch_ext;
mod impl_pitch_rand;
mod impl_pitch_repr;

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
