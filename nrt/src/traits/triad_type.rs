/*
    Appellation: triad_kind <module>
    Created At: 2025.12.23:14:15:46
    Contrib: @FL03
*/
/// The [`Relative`] trait is used to define a relationship between two distinct triad
/// classifications
pub trait Relative {
    type Rel;

    private! {}

    fn rel(&self) -> Self::Rel;
}

/// The [`TriadType`] trait is used to represent the various classifications of a triad
/// considered by the Neo-Riemannian theory.
pub trait TriadType: Relative
where
    Self: 'static + Copy + Default + Relative + Send + Sync + core::fmt::Debug + core::fmt::Display,
{
    private! {}

    fn new() -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    fn root(&self) -> usize;

    fn fifth(&self) -> usize;

    fn third(&self) -> usize;

    fn is_major(&self) -> bool {
        false
    }

    fn is_minor(&self) -> bool {
        false
    }

    fn is_augmented(&self) -> bool {
        false
    }

    fn is_diminished(&self) -> bool {
        false
    }
}

/*
 ************* Implementations *************
*/

macro_rules! triad_kind {
    (impl $trait:ident for {$($($name:ident)::*<Rel = $rel:ty>::<[$($v:literal),* $(,)?]> $({$($rest:tt)*})?),* $(,)?}) => {
        $(triad_kind! { @impl $trait for $($name)::*<Rel = $rel>::<[$($v),*]> $({$($rest)*})? })*
    };
    (@impl $trait:ident for $($name:ident)::* <Rel = $rel:ty>::<[$r:literal, $f:literal, $t:literal]> $({$($rest:tt)*})?) => {
        impl $trait for $($name)::* {
            // type Rel = $rel;

            seal! {}

            fn root(&self) -> usize {
                $r
            }

            fn fifth(&self) -> usize {
                $f
            }

            fn third(&self) -> usize {
                $t
            }

            $($($rest)*)?
        }

        impl Relative for $($name)::* {
            type Rel = $rel;

            seal! {}

            fn rel(&self) -> Self::Rel {
                <$rel>::default()
            }
        }
    };
}

use rstmt_core::{Augmented, Diminished, Major, Minor};

triad_kind! {
    impl TriadType for {
        Augmented<Rel = Diminished>::<[4, 8, 4]> {
            fn is_augmented(&self) -> bool {
                true
            }
        },
        Diminished<Rel = Augmented>::<[3, 6, 3]> {
            fn is_diminished(&self) -> bool {
                true
            }
        },
        Major<Rel = Minor>::<[4, 7, 3]> {
            fn is_major(&self) -> bool {
                true
            }
        },
        Minor<Rel = Major>::<[3, 7, 4]> {
            fn is_minor(&self) -> bool {
                true
            }
        }
    }
}
