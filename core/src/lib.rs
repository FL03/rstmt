/*
    Appellation: rstmt-core <library>
    Contrib: @FL03
*/
//! This crate provides the core functionality for the `rstmt` library, including [`Aspn`],
//! [`NoteBase`], [`Pitch`], and [`Octave`]. Additionally, the crate provides a host of
//! other primitives and utilities designed to manifest and manipulate musical concepts.
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::should_implement_trait,
    clippy::upper_case_acronyms
)]
#![cfg_attr(not(feature = "std"), no_std)]

// compiler check
#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error! { "either the \"std\" or \"alloc\" feature must be enabled" }
// procedural macros
#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
}

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod error;
pub mod freq;
pub mod intervals;
pub mod notes;
pub mod octave;
pub mod pitch;

pub mod consts {
    //! this module implements various constants used throughout the library.
    //!

    /// The C Major scale represented as an array of pitch class indices.
    pub const C_MAJOR_SCALE: [usize; 7] = [0, 2, 4, 5, 7, 9, 11];
}

pub mod types {
    //! this module imimplements various types and other primitives used throughout the library
    #[doc(inline)]
    pub use self::{accidentals::*, harmonic_funcs::*};

    mod accidentals;
    mod harmonic_funcs;
}
// re-exports
#[doc(inline)]
pub use self::{
    consts::*,
    error::*,
    freq::{AsFrequency, Frequency, IntoFrequency, ScaleToFrequency},
    notes::*,
    octave::*,
    pitch::{AsPitch, ClassifiedPitch, IntoPitch, PitchCls, RawPitch},
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

    pub use crate::consts::*;
    pub use crate::types::*;

    pub use crate::freq::*;
    pub use crate::intervals::prelude::*;
    pub use crate::notes::*;
    pub use crate::octave::*;
    pub use crate::pitch::prelude::*;
}
