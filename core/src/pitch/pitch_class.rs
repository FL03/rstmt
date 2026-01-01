/*
    Appellation: pitch_class <module>
    Created At: 2025.12.20:09:31:05
    Contrib: @FL03
*/
use crate::pitch::{RawAccidental, RawPitchClass};

/// The [`PitchClass`] implementations works to generically define the structure for a pitch
/// class. This is accomplished through the use of two type parameters: `N`, which defines the
/// note (e.g., C, D, E, etc.), and `K`, which defines the kind of pitch (e.g., sharp, flat,
/// natural, etc.).
///
/// **Note**: This struct isn't designed to be used directly, rather through type aliases such
/// as [`C`], [`DSharp`], [`EFlat`], etc.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[repr(C)]
pub struct PitchClass<P = super::CNote, K = <P as RawPitchClass>::Tag>
where
    P: RawPitchClass<Tag = K>,
    K: RawAccidental,
{
    pub(crate) class: P,
    pub(crate) kind: K,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pitch_class() {
        let c = C::default();
        // verify the type checkers
        assert! { c.is_natural() && !c.is_flat() && !c.is_sharp() }
        // check the index
        assert_eq! { c, 0 }
    }

    #[test]
    fn test_parse_pitch_class() {
        let d_sharp: DSharp = "D#".parse().unwrap();
        assert! { d_sharp.is_sharp() }
        assert_eq! { d_sharp.index(), 3 }

        let e_flat: EFlat = "Eb".parse().unwrap();
        assert! { e_flat.is_flat() }
        assert_eq! { e_flat.index(), 3 }

        let f: F = "F".parse().unwrap();
        assert! { f.is_natural() }
        assert_eq! { f.index(), 5 }
    }
}
