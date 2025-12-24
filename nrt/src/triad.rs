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
//! assert_eq! { triad, [0, 4, 7] }
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

use crate::traits::{RawTriad, TriadCls};
use crate::types::TriadClass;
use rstmt_core::{Major, Octave, RawChord};

/// The standard alias for a dynamic triad representation.
pub type Triad = TriadBase<[usize; 3], TriadClass, usize>;

/// The [`TriadBase`] is an implementation of a triad generic over the chord, or storage, its
/// classification, and the element type used to represent a note within the triadic chord.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
pub struct TriadBase<S = [usize; 3], K = Major, T = <S as RawChord>::Elem>
where
    K: TriadCls,
    S: RawTriad<Elem = T>,
{
    pub(crate) chord: S,
    pub(crate) class: K,
    pub(crate) octave: Octave,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test: test initialization routines using a single root note, `C(0)`
    fn test_traid_init_c_root() {
        assert_eq! { Triad::major(0), [0, 4, 7] }
        assert_eq! { Triad::minor(0), [0, 3, 7] }
        assert_eq! { Triad::augmented(0), [0, 4, 8] }
        assert_eq! { Triad::diminished(0), [0, 3, 6] }
    }

    #[test]
    /// Test: test initialization routines using various root notes
    fn test_triad_init_n_root() {
        assert_eq! { Triad::major(1), [1, 5, 8] }
        assert_eq! { Triad::minor(2), [2, 5, 9] }
        assert_eq! { Triad::augmented(6), [6, 10, 2] }
        assert_eq! { Triad::diminished(11), [11, 2, 5] }
    }
}
