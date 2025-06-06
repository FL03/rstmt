/*
    Appellation: rstmt-core <library>
    Contrib: @FL03
*/
//! # rstmt-core
//!
//! This crate provides the core functionality for the `rstmt` library.
//!
#![cfg_attr(not(feature = "std"), no_std)]
#![crate_name = "rstmt_core"]
#![crate_type = "lib"]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::{
    error::*,
    notes::{AsNote, IntoNote, Note, NoteBase, Octave},
    pitch::{Pitch, PitchClass, RawPitch},
    traits::prelude::*,
    types::prelude::*,
};

#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
}

pub mod error;
pub mod intervals;
pub mod notes;
pub mod pitch;

pub mod traits {
    //! this module implements the core traits used throughout the library.
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod convert;
    pub mod num;

    pub(crate) mod prelude {
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
    pub use crate::error::*;
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
