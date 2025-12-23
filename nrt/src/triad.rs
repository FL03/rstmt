/*
    Appellation: triad <module>
    Created At: 2025.12.20:11:38:53
    Contrib: @FL03
*/
//! this module defines the [`Triad`] struct along with additional types and traits supporting
//! the representation of triads and their operations w.r.t. the neo-riemannian theory.
//!
//! # Overview
//!
//! ```rust
//! use rstmt_nrt::Triad;
//!
//! // initialize a c-major triad: (0, 4, 7)
//! let triad = Triad::major(0);
//! // verify the composition
//! assert_eq!(triad.root(), 0);
//! assert_eq!(triad.third(), 4);
//! assert_eq!(triad.fifth(), 7);
//! assert!(triad.is_major());
//! ```
//!
//! # Background
//!
//! A triad is defined to be a chord, composed of three notes, each of which maintain certain
//! intervallic relationships with one another. More specifically, the distance between the
//! first and second as well as the second and third notes is defined to be a major or minor
//! third, whilst the distance between the first and third notes is some variant of a _fifth_.
//!
//! # References
//!
//! - [Continuous Transformations](https://www.mtosmt.org/issues/mto.04.10.3/mto.04.10.3.callender.pdf)
//! - [Neo-Riemannian Theory](https://en.wikipedia.org/wiki/Neo-Riemannian_theory)//!

use crate::traits::{RawTriadStore, TriadCls};
use crate::types::TriadClass;
use rstmt_core::{Major, Octave};

/// A triad is a particular chord composed of three notes that satify particular intervallic
/// constrains with each other. Here, the triad materializes the facet of a hyperedge within a
/// cluster of triads persisted in the Tonnetz. The triad is a fundamental entity in the
/// substrate used to represent the _headspace_ of a plant. Each plant relies on these objects
/// to transverse the surface of the tonnetz so that it may gaurantee the completion of a task.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Triad {
    /// The type of triad (Major, Minor, Augmented, Diminished)
    pub(crate) class: TriadClass,
    /// the set of three pitch classes defining the triad
    pub(crate) notes: [usize; 3],
    /// The octave of the triad
    pub(crate) octave: Octave,
}

/// The [`TriadBase`] implementation is a generic representation of a triad.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, rename_all = "snake_case")
)]
pub struct TriadBase<S = [usize; 3], K = Major, T = <S as RawTriadStore>::Elem>
where
    K: TriadCls,
    S: RawTriadStore<Elem = T>,
{
    pub(crate) chord: S,
    pub(crate) class: K,
}
