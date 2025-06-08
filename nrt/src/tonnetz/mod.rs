/*
    appellation: tonnetz <module>
    authors: @FL03
*/
//! # tonnetz
//!
//! A tonnetz is generally understood to be a geometrical interpretation of tonal space, where
//! each facet represents a valid triad.
//!
#[cfg(feature = "std")]
#[doc(inline)]
pub use self::hash_tonnetz::HashTonnetz;

#[cfg(feature = "std")]
pub mod hash_tonnetz;

pub(crate) mod prelude {
    #[cfg(feature = "std")]
    #[doc(inline)]
    pub use super::hash_tonnetz::*;
}

#[deprecated(
    since = "0.0.5",
    note = "Please use `hash_tonnetz::HashTonnetz` instead."
)]
pub type Tonnetz = HashTonnetz;
