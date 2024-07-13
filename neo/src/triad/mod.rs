/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{builder::TriadBuilder, classes::Triads, triad::Triad};

pub(crate) mod builder;
pub(crate) mod triad;

pub mod classes;

pub(crate) mod prelude {
    pub use super::builder::TriadBuilder;
    pub use super::classes::*;
    pub use super::triad::Triad;
}

use rstmt::Intervals;

pub trait Triadic<N> {
    fn intervals(&self) -> impl Iterator<Item = Intervals>;

    fn kind(&self) -> Triads;

    fn notes(&self) -> (N, N, N);
}

pub trait TriadData {
    type Elem;

    fn root(&self) -> &Self::Elem;

    fn third(&self) -> &Self::Elem;

    fn fifth(&self) -> &Self::Elem;
}
