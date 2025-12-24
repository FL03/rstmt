/*
    Appellation: rstmt-core <library>
    Contrib: @FL03
*/
//! Various traits and interfaces designed to enable the development of a type-base musical
//! framework.
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
// macros
#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
}
// external crates
#[cfg(feature = "alloc")]
extern crate alloc;
// modules
pub mod chord;
pub mod chroma;
pub mod classify;
pub mod num;

pub mod ops {
    //! This module provides various operations traits and implementations for musical concepts
    #[doc(inline)]
    pub use self::{modulo::*, transform::*};

    mod modulo;
    mod transform;
}

// re-exports
#[doc(inline)]
pub use self::prelude::*;
// prelude
#[doc(hidden)]
pub mod prelude {
    #[doc(inline)]
    pub use crate::chord::*;
    #[doc(inline)]
    pub use crate::chroma::*;
    #[doc(inline)]
    pub use crate::classify::*;
    #[doc(inline)]
    pub use crate::num::*;
    #[doc(inline)]
    pub use crate::ops::*;
}
