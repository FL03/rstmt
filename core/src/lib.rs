#![crate_name = "rstmt_core"]
//! This crate provides the core functionality for the `rstmt` library, including [`Aspn`],
//! [`NoteBase`], [`Pitch`], and [`Octave`]. Additionally, the crate provides a host of
//! other primitives and utilities designed to manifest and manipulate musical concepts.
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
#![cfg_attr(feature = "nightly", feature(allocator_api))]
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

pub mod chord;
pub mod comp;
pub mod consts;
pub mod error;
pub mod freq;
pub mod intervals;
pub mod note;
pub mod octave;
pub mod pitch;

pub mod types {
    //! this module imimplements various types and other primitives used throughout the library
    #[doc(inline)]
    pub use self::{accents::*, harmonic_funcs::*, notes::*};

    mod accents;
    mod harmonic_funcs;
    mod notes;
}

pub mod utils {
    //! useful utilities for musical primitives for converting between different
    //! representations, classification routines, and more.
    #[doc(inline)]
    pub use self::frequency::*;

    mod frequency;
}
// re-exports
#[doc(inline)]
pub use self::{
    chord::{RawChord, RawChordMut},
    comp::Scale,
    consts::*,
    error::*,
    freq::*,
    intervals::*,
    note::*,
    octave::*,
    pitch::*,
    types::*,
    utils::*,
};
#[doc(inline)]
pub use rstmt_traits as traits;
#[doc(inline)]
pub use rstmt_traits::prelude::*;
// prelude
#[doc(hidden)]
pub mod prelude {
    pub use rstmt_traits::prelude::*;

    pub use crate::chord::prelude::*;
    pub use crate::comp::prelude::*;
    pub use crate::consts::*;
    pub use crate::freq::*;
    pub use crate::intervals::prelude::*;
    pub use crate::note::*;
    pub use crate::octave::*;
    pub use crate::pitch::prelude::*;
    pub use crate::types::*;
    pub use crate::utils::*;
}
