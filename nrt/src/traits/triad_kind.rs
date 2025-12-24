/*
    Appellation: triad_kind <module>
    Created At: 2025.12.23:14:15:46
    Contrib: @FL03
*/

/// [`TriadCls`] is a trait that represents the kind of a triad in Neo-Riemannian theory.
pub trait TriadCls:
    'static + Copy + Default + Send + Sync + core::fmt::Debug + core::fmt::Display
{
    private!();

    fn new() -> Self
    where
        Self: Sized,
    {
        Self::default()
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
    ($($($name:ident)::*.$method:ident($($v:literal),* $(,)?)),* $(,)?) => {
        $(triad_kind! { @impl TriadCls for $($name)::*.$method($($v),*) })*
    };
    (@impl $trait:ident for $($name:ident)::* .$method:ident($r:literal, $f:literal, $t:literal)) => {
        impl $trait for $($name)::* {
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

triad_kind! {
    rstmt_core::Augmented.is_augmented(4, 8, 4),
    rstmt_core::Diminished.is_diminished(3, 6, 3),
    rstmt_core::Major.is_major(4, 7, 3),
    rstmt_core::Minor.is_minor(3, 7, 4)
}
