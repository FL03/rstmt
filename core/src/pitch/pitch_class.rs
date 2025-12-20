/*
    Appellation: pitch_class <module>
    Created At: 2025.12.20:09:31:05
    Contrib: @FL03
*/
use super::{CNote, Natural, PitchCls, PitchType};

/// The [`PitchClass`] implementations works to generically define the structure for a pitch
/// class. This is accomplished through the use of two type parameters: `N`, which defines the
/// note (e.g., C, D, E, etc.), and `K`, which defines the kind of pitch (e.g., sharp, flat,
/// natural, etc.).
///
/// **Note**: This struct isn't designed to be used directly, rather through type aliases such
/// as [`C`](super::C), [`DSharp`](super::DSharp), [`EFlat`](super::EFlat), etc.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[repr(C)]
pub struct PitchClass<T = CNote, K = Natural>
where
    T: PitchCls<Tag = K>,
    K: PitchType,
{
    pub(crate) class: T,
    pub(crate) _marker: core::marker::PhantomData<K>,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[repr(transparent)]
pub struct ConstPitchClass<const N: usize>;
