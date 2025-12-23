/*
    Appellation: pitch_class <module>
    Created At: 2025.12.20:09:31:05
    Contrib: @FL03
*/
use super::{Accidental, CNote, Natural, PitchRepr};

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
pub struct PitchClass<N = CNote, A = Natural>
where
    N: PitchRepr<Tag = A>,
    A: Accidental,
{
    pub(crate) class: N,
    pub(crate) _marker: core::marker::PhantomData<A>,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[repr(transparent)]
pub struct ConstPitchClass<const N: usize, A = Natural>
where
    A: Accidental,
{
    pub(crate) _marker: core::marker::PhantomData<A>,
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
