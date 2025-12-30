#![crate_name = "rstmt_traits"]
//! A collection of useful traits focused on musical abstractions, composition, and operations.
//!
//!
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
pub mod chroma;
pub mod classify;
pub mod num;

pub mod ops {
    //! This module provides various operations traits and implementations for musical concepts
    #[doc(inline)]
    pub use self::{apply::*, pymod::*, transform::*};

    mod apply;
    mod pymod;
    mod transform;
}

// re-exports
#[doc(inline)]
pub use self::{chroma::*, classify::*, num::*, ops::*};
// prelude
#[doc(hidden)]
pub mod prelude {
    pub use crate::chroma::*;
    pub use crate::classify::*;
    pub use crate::num::*;
    pub use crate::ops::*;
}
