//! This crate provides the core functionality for the `rstmt` library, including [`Aspn`],
//! [`NoteBase`], [`Pitch`], and [`Octave`]. Additionally, the crate provides a host of
//! other primitives and utilities designed to manifest and manipulate musical concepts.
//!
//! ## Overview
//!
//! The core modules focus on establishing the basic primitives and interfaces needed to
//! represent musical notes, pitches, octaves, and related concepts. These modules
//! provide the foundational building blocks for more complex musical structures and
//! operations.
//!
//! These modules are designed to be efficient, flexible, and correct, ensuring conversions
//! between different representations are handled seamlessly. For example, _any_ [`PitchClass`]
//! is able to be converted directly into a [`Frequency`].
#![crate_name = "rstmt_core"]
#![crate_type = "lib"]
#![allow(
    clippy::derivable_impls,
    clippy::len_without_is_empty,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::non_canonical_partial_ord_impl,
    clippy::should_implement_trait,
    clippy::upper_case_acronyms
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(feature = "alloc", feature = "nightly"), feature(allocator_api))]
// compiler check
#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error! { "either the \"std\" or \"alloc\" feature must be enabled" }
// procedural macros
#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
    #[macro_use]
    pub mod units;
}

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod chords;
pub mod compose;
pub mod consts;
pub mod error;
pub mod freq;
pub mod intervals;
pub mod notes;
pub mod octave;
pub mod pitch;

pub mod types {
    //! this module imimplements various types and other primitives used throughout the library
    #[doc(inline)]
    pub use self::harmonic_funcs::*;

    mod harmonic_funcs;
}

// re-exports
#[doc(inline)]
pub use self::{
    chords::{RawChord, RawChordMut},
    compose::Scale,
    consts::*,
    error::*,
    freq::{Frequency, RawFrequency},
    intervals::*,
    notes::*,
    octave::*,
    pitch::*,
    types::*,
};
#[doc(inline)]
pub use rstmt_traits as traits;
#[doc(inline)]
pub use rstmt_traits::prelude::*;
// prelude
#[doc(hidden)]
pub mod prelude {
    pub use rstmt_traits::prelude::*;

    pub use crate::chords::prelude::*;
    pub use crate::compose::prelude::*;
    pub use crate::consts::*;
    pub use crate::freq::prelude::*;
    pub use crate::intervals::prelude::*;
    pub use crate::notes::prelude::*;
    pub use crate::octave::*;
    pub use crate::pitch::prelude::*;
    pub use crate::types::*;
}
