/*
    Appellation: pitch_class <module>
    Created At: 2025.12.20:09:31:05
    Contrib: @FL03
*/
use super::{Accidental, CNote, Natural, PitchClassRepr};

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
pub struct PitchClass<P = CNote, K = Natural>
where
    P: PitchClassRepr<Tag = K>,
    K: Accidental,
{
    pub(crate) class: P,
    pub(crate) kind: K,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[repr(transparent)]
pub struct ConstPitchClass<const N: usize, P, K>
where
    P: PitchClassRepr<Tag = K>,
    K: Accidental,
{
    pub(crate) _class: PitchClass<P, K>,
}

/*
 ************* Types *************
*/
macro_rules! classes {
    (@impl $name:ident::<Natural>) => {
        paste::paste! {
            pub type $name = $crate::pitch::PitchClass<$crate::pitch::[<$name Note>], $crate::pitch::Natural>;
        }
    };
    (@impl $name:ident::<$kind:ident>) => {
        paste::paste! {
            pub type [<$name $kind>] = $crate::pitch::PitchClass<$crate::pitch::[<$name $kind Note>], $crate::pitch::$kind>;
        }
    };
    (@impl $name:ident::<$($kind:ident),+ $(,)?>) => {
        $(classes! { @impl $name::<$kind> })*
    };
    ($($name:ident::<$($K:ident),* $(,)?>),* $(,)?) => {
        $(classes! { @impl $name::<Natural, $($K),*> })*
    };
}

classes! {
    C::<Sharp>,
    D::<Flat, Sharp>,
    E::<Flat>,
    F::<Sharp>,
    G::<Flat, Sharp>,
    A::<Flat, Sharp>,
    B::<Flat>,
}
