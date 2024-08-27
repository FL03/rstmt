/*
    Appellation: tonnetz <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Tonnetz
//!
//! The tonnetz is defined to be a conceptual lattice diagram that represents tonal space.
//!
//! For our purposes, the tonnetz is notable in its ability to represent the space between
//! persistent triadic objects as well as their path through time within the same space.
//!
//! ### Background
//!
//! The tonnetz was first introduced by Leonhard Euler in 1739 as a way to visualize the
//! relationships between the notes of the major and minor triads.
//!
//! In 2012, Dmitri Tymoczko successfully generalized the structure of the tonnetz to include
//! all possible triads.
//!
//! ### Concepts
//!
//! Considering a triad is a 2-simplex, the tonnetz may be formally described as a simplicial
//! complex which glues together the various triads in a way that is consistent with the
//! relationships between the notes.
//!

pub(crate) mod prelude {
    pub use super::Tonnetze;
}

pub trait Tonnetze {
    fn is_complete(&self) -> bool;

    fn layers(&self) -> usize;
}
