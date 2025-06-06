/*
    appellation: triad <module>
    authors: @FL03
*/
//! this module implements the triad
#[doc(inline)]
pub use self::{class::*, factors::*, triad::Triad};

pub mod class;
pub mod factors;
pub mod triad;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::class::*;
    #[doc(inline)]
    pub use super::factors::*;
    #[doc(inline)]
    pub use super::triad::*;
}
