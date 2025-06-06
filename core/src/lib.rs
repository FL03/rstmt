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
    clippy::module_inception
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![crate_name = "rstmt_core"]
#![crate_type = "lib"]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::{
    consts::*,
    error::*,
    freq::{Frequency, RawFrequency},
    notes::{AsAspn, Aspn, IntoAspn, NoteBase, Octave},
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

    pub mod chroma;
    pub mod convert;
    pub mod num;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::chroma::*;
        #[doc(inline)]
        pub use super::convert::*;
        #[doc(inline)]
        pub use super::num::*;
    }
}

pub mod types {
    //! this module imimplements various types and other primitives used throughout the library
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod harmonic_funcs;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::harmonic_funcs::*;
    }
}

pub mod prelude {
    #[doc(no_inline)]
    pub use crate::consts::*;
    #[doc(no_inline)]
    pub use crate::error::*;

    #[doc(no_inline)]
    pub use crate::freq::prelude::*;
    #[doc(inline)]
    pub use crate::intervals::prelude::*;
    #[doc(no_inline)]
    pub use crate::notes::prelude::*;
    #[doc(no_inline)]
    pub use crate::pitch::prelude::*;
    #[doc(no_inline)]
    pub use crate::traits::prelude::*;
    #[doc(no_inline)]
    pub use crate::types::prelude::*;
}
