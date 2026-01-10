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
//! A triad is defined to be a chord, composed of three notes, each of which maintain certain
//! intervallic relationships with one another. More specifically, the distance between the
//! first and second as well as the second and third notes is defined to be a major or minor
//! third, whilst the distance between the first and third notes is some variant of a _fifth_.
//! 
//! These compositional contraints lead to four possible triadic chord types, namely: major, 
//! minor, augmented, and diminished. Each of these chord types can be represented as a 3-tuple
//! containing the pitch classes of each note within the chord. For example, a C-major triad 
//! can be represented as the tuple (0, 4, 7), where `0` represents the root note C, `4` the 
//! major third E, and `7` as the perfect fifth G.
//! 
//! Additionally, triads may be transformed using any one of three transformations defined as: 
//! leading, parallel, and relative. The behavior of these transformations is determined by the
//! interval between the first two notes of the triad, i.e. a major or minor third. Each of 
//! these transformations is capable of being chained together into discrete and continuous 
//! sequences or spaces and is its own inverse. This means that any consecutive applications of
//! any one particular transformation simply reverts the object back into its original state. 
//! 
//! That being said, augmented and diminished traids _can_ be transformed, however, the exact 
//! nature of their responses is not as predictable as with major / minor triads.  
//! 
//! ## Examples
//!
//! ### _Basic Usage_
//! 
//! Initialize a C-major triad and verify its composition
//! 
//! ```rust
//! use rstmt_nrt::Triad;
//! // create new triad
//! let c_major = Triad::major(0);
//! // verify the composition
//! assert_eq! { c_major, [0, 4, 7] }
//! ```
//! 
//! ### _Transformations_
//! 
//! Initialize a C-major triad before applying each of the three transformations:
//! 
//! ```rust
//! use rstmt_nrt::Triad;
//! // create new triad
//! let c_major = Triad::major(0);
//! // apply leading transformation
//! assert_eq! { c_major.leading(), Triad::minor(4) }
//! // apply parallel transformation
//! assert_eq! { c_major.parallel(), Triad::minor(0) }
//! // apply relative transformation
//! assert_eq! { c_major.relative(), Triad::minor(9) }
//! // confirm inverses
//! assert_eq! { c_major.leading().leading(), c_major }
//! assert_eq! { c_major.parallel().parallel(), c_major }
//! assert_eq! { c_major.relative().relative(), c_major }
//! ```
//!
//! # References
//!
//! - [Continuous Transformations](https://www.mtosmt.org/issues/mto.04.10.3/mto.04.10.3.callender.pdf)
//! - [Neo-Riemannian Theory](https://en.wikipedia.org/wiki/Neo-Riemannian_theory)

use crate::traits::{TriadRepr, TriadType};
use crate::types::Triads;
use rspace_traits::RawSpace;
use rstmt_core::{Major, Octave};

/// The default representation of a triadic chord
pub type TriChord<T = isize> = [T; 3];
/// A type alias for a [`TriadBase`] instance configured to use the [`TriChord`] as
/// its storage
pub type Triad<K = Triads, T = usize> = TriadBase<TriChord<T>, K, T>;
/// A type alias for a [`Triad`] using a dynamic classifier of type [`Triads`]
pub type DynTriad<T = usize> = Triad<Triads, T>;

/// The [`TriadBase`] is an implementation of a triad generic over the chord, or storage, its
/// classification, and the element type used to represent a note within the triadic chord.
#[derive(Clone, Copy, Default, Eq, Hash, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
pub struct TriadBase<S = TriChord, K = Major, T = <S as RawSpace>::Elem>
where
    K: TriadType,
    S: TriadRepr<Elem = T>,
{
    pub(crate) chord: S,
    pub(crate) class: K,
    pub(crate) octave: Octave<T>,
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
