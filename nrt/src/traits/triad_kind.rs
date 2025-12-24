/*
    Appellation: triad_kind <module>
    Created At: 2025.12.23:14:15:46
    Contrib: @FL03
*/

/// The [`TriadCls`] trait is used to represent the various classifications of a triad
/// considered by the Neo-Riemannian theory.
pub trait TriadCls:
    'static + Copy + Default + Send + Sync + core::fmt::Debug + core::fmt::Display
{
    type Rel: TriadCls<Rel = Self>;

    private! {}

    fn new() -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    fn rel(&self) -> Self::Rel {
        Self::Rel::new()
    }

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

    fn root(&self) -> usize;

    fn fifth(&self) -> usize;

    fn third(&self) -> usize;
}

/*
 ************* Implementations *************
*/

macro_rules! triad_kind {
    ($($($name:ident)::*<Rel = $rel:ty>.$method:ident($($v:literal),* $(,)?)),* $(,)?) => {
        $(triad_kind! { @impl TriadCls for $($name)::*<Rel = $rel>.$method($($v),*) })*
    };
    (@impl $trait:ident for $($name:ident)::* <Rel = $rel:ty>.$method:ident($r:literal, $f:literal, $t:literal)) => {
        impl $trait for $($name)::* {
            type Rel = $rel;

            seal! {}

            fn $method(&self) -> bool {
                true
            }

            fn root(&self) -> usize {
                $r
            }

            fn fifth(&self) -> usize {
                $f
            }

            fn third(&self) -> usize {
                $t
            }
        }
    };
}

use rstmt_core::{Augmented, Diminished, Major, Minor};

triad_kind! {
    Augmented<Rel = Diminished>.is_augmented(4, 8, 4),
    Diminished<Rel = Augmented>.is_diminished(3, 6, 3),
    Major<Rel = Minor>.is_major(4, 7, 3),
    Minor<Rel = Major>.is_minor(3, 7, 4)
}
