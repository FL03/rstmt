/*
    appellation: tonnetz <module>
    authors: @FL03
*/
//! # tonnetz
//!
//! The tonnetz is a geometric representation of tonal space, where each facet represents a
//! triad. This model was first introduced by Leonhard Euler in 1739
//!
//! A tonnetz is generally understood to be a geometrical interpretation of tonal space, where
//! each facet represents a valid triad.
//!
//! ## Resources
//!
//! Listed below are some useful resources for understanding the tonnetz and its potential
//! applications in both music theroy as well as computer science:
//!
//! - [Wikipedia: Tonnetz](https://en.wikipedia.org/wiki/Tonnetz)
//! - [The Generalized Tonnetz](https://dmitri.mycpanel.princeton.edu/tonnetzes.pdf)
//!
#[cfg(feature = "std")]
#[doc(inline)]
pub use self::hyper_tonnetz::HyperTonnetz;

#[cfg(feature = "std")]
pub mod hyper_tonnetz;

pub(crate) mod prelude {
    #[cfg(feature = "std")]
    #[doc(inline)]
    pub use super::hyper_tonnetz::*;
}

#[doc(hidden)]
#[deprecated(
    since = "0.0.5",
    note = "use `HyperTonnetz` instead; this will be removed in the next major release."
)]
pub type HashTonnetz = HyperTonnetz;
