/*
    Appellation: triad_kind <module>
    Created At: 2025.12.23:14:15:46
    Contrib: @FL03
*/

/// [`TriadCls`] is a trait that represents the kind of a triad in Neo-Riemannian theory.
pub trait TriadCls: 'static + Copy + Send + Sync + core::fmt::Debug + core::fmt::Display {
    private!();

    fn new() -> Self;
}

/*
 ************* Implementations *************
*/

macro_rules! triad_kind {
    ($($($name:ident)::*),* $(,)?) => {
        $(triad_kind! { @impl TriadCls for $($name)::* })*
    };
    (@impl $trait:ident for $($name:ident)::*) => {
        impl $trait for $($name)::* {
            seal! {}

            fn new() -> Self {
                Self::default()
            }
        }
    };
}

triad_kind! {
    rstmt_core::Augmented,
    rstmt_core::Diminished,
    rstmt_core::Major,
    rstmt_core::Minor
}
