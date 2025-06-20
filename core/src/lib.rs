/*
    Appellation: rstmt-core <library>
    Contrib: @FL03
*/
//! # rstmt-core
//!
//! This crate provides the core functionality for the `rstmt` library, including [`Aspn`],
//! [`NoteBase`], [`Pitch`], and [`Octave`]. Additionally, the crate provides a host of
//! other primitives and utilities designed to manifest and manipulate musical concepts.
#![allow(
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::should_implement_trait
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]
#![crate_type = "lib"]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::{
    consts::*,
    error::*,
    freq::{Frequency, RawFrequency},
    notes::{AsAspn, Aspn, IntoAspn, NoteBase},
    octave::{AsOctave, IntoOctave, Octave},
    pitch::{AsPitch, IntoPitch, Pitch, PitchClass, RawPitch},
    traits::prelude::*,
    types::prelude::*,
};

#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
}

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

pub mod traits {
    //! this module implements the core traits used throughout the library.
    #[doc(inline)]
    pub use self::prelude::*;

    mod chroma;
    mod num;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::chroma::*;
        #[doc(inline)]
        pub use super::num::*;
    }
}

pub mod types {
    //! this module imimplements various types and other primitives used throughout the library
    #[doc(inline)]
    pub use self::prelude::*;

    mod harmonic_funcs;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::harmonic_funcs::*;
    }
}

#[doc(hidden)]
pub mod prelude {
    pub use crate::consts::*;
    pub use crate::traits::*;
    pub use crate::types::*;

    pub use crate::freq::prelude::*;
    pub use crate::intervals::prelude::*;
    pub use crate::notes::prelude::*;
    pub use crate::octave::prelude::*;
    pub use crate::pitch::prelude::*;
}
