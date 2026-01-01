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
//! let c_major = Triad::major(0);
//! // verify the composition
//! assert_eq! { c_major, [0, 4, 7] }
//! assert! { c_major.is_major() && !c_major.is_minor() }
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
//! - [Neo-Riemannian Theory](https://en.wikipedia.org/wiki/Neo-Riemannian_theory)

use crate::traits::{TriadRepr, TriadType};
use crate::types::Triads;
use rspace_traits::RawSpace;
use rstmt_core::{Major, Octave};

/// A type alias for a [`TriadBase`] instance configured to use the [`DefaultTriadChord`] as
/// its storage
pub type Triad<K = Triads, T = usize> = TriadBase<DefaultTriadChord<T>, K, T>;

pub type DynTriad<T = usize> = Triad<Triads, T>;
/// The default representation of a triadic chord
pub type DefaultTriadChord<T = isize> = [T; 3];

/// The [`TriadBase`] is an implementation of a triad generic over the chord, or storage, its
/// classification, and the element type used to represent a note within the triadic chord.
#[derive(Clone, Copy, Default, Eq, Hash, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
pub struct TriadBase<S = DefaultTriadChord, K = Major, T = <S as RawSpace>::Elem>
where
    K: TriadType,
    S: TriadRepr<Elem = T>,
{
    pub(crate) chord: S,
    pub(crate) class: K,
    pub(crate) octave: Octave,
}

#[cfg(test)]
mod tests {
    use super::Triad;

    #[test]
    /// Test: test initialization routines using a single root note, `C(0)`
    fn test_triad_create() {
        assert_eq! { Triad::major(0), [0, 4, 7] }
        assert_eq! { Triad::minor(0), [0, 3, 7] }
        assert_eq! { Triad::augmented(0), [0, 4, 8] }
        assert_eq! { Triad::diminished(0), [0, 3, 6] }
    }

    #[test]
    /// Test: test initialization routines using various root notes
    fn test_triad_create_with_n() {
        for root in 0..12 {
            let major = Triad::major(root);
            let minor = Triad::minor(root);
            let augmented = Triad::augmented(root);
            let diminished = Triad::diminished(root);
            assert! { major.is_major() && !major.is_minor() }
            assert! { minor.is_minor() && !minor.is_major() }
            assert! { augmented.is_augmented() }
            assert! { diminished.is_diminished() }
        }
    }

    #[test]
    fn test_triad_properties() {
        let fsharp_minor = Triad::minor(6);
        assert! { fsharp_minor.is_minor() && !fsharp_minor.is_major() }
        assert_eq! { fsharp_minor.class(), rstmt_core::Minor }
    }

    #[test]
    fn test_triad_transform_c_major() {
        let c_major = Triad::major(0);
        let leading = Triad::minor(4);
        let parallel = Triad::minor(0);
        let relative = Triad::minor(9);
        // leading
        assert! {
            c_major.leading() == leading &&
            leading.leading() == c_major &&
            c_major.leading().leading() == leading.leading()
        }
        // parallel
        assert! {
            c_major.parallel() == parallel &&
            parallel.parallel() == c_major &&
            c_major.parallel().parallel() == parallel.parallel()
        }
        // relative
        assert! {
            c_major.relative() == relative &&
            relative.relative() == c_major &&
            c_major.relative().relative() == relative.relative()
        }
    }
}
