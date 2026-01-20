//! A collection of useful traits focused on musical abstractions, composition, and operations.
//!
//!
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
    pub use self::{precision::*, pymod::*, transform::*};

    mod precision;
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
