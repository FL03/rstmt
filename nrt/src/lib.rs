/*
    Appellation: rstmt-nrt <library>
    Contrib: @FL03
*/
//! # rstmt-nrt
//!
//! This crate works to establish a solid foundation fo exploring the neo-riemannian theory,
//! providing implementations of the [`Triad`], its transformations [`LPR`], and the
//! generalized tonnetz (`Tonnetz`).
//!
//! ## Background
//!
//! Before diving into the implementation details, it is important to understand the concepts
//! at hand and the theory behind them.
//!
//! ### Neo-Riemannian Theory
//!
//! The neo-riemannian theory is a loose collection of musical theories focused on the triad.
#![allow(
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::non_canonical_partial_ord_impl,
    clippy::should_implement_trait
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]
#![crate_type = "lib"]

#[cfg(feature = "alloc")]
extern crate alloc;

/// re-declare the external `rstmt_core` crate as `rstmt` for convenience
extern crate rstmt_core as rstmt;

#[doc(inline)]
pub use self::{
    error::*,
    triad::{Triad, Triads},
    types::prelude::*,
};

#[cfg(feature = "std")]
pub use self::transform::TriadNavigator;

#[cfg(feature = "tonnetz")]
#[doc(inline)]
pub use self::{tonnetz::HyperTonnetz, transform::MotionPlanner};

#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
}
/// this module defines the standard error type, [`TriadError`], for the crate
pub mod error;
#[cfg(feature = "tonnetz")]
pub mod tonnetz;
pub mod transform;
pub mod triad;

pub mod types {
    //! this module defines various types supporting the neo-riemannian theory
    #[doc(inline)]
    pub use self::prelude::*;

    mod lpr;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::lpr::*;
    }
}

pub mod prelude {
    #[cfg(feature = "tonnetz")]
    pub use crate::tonnetz::prelude::*;
    #[cfg(feature = "alloc")]
    pub use crate::transform::prelude::*;
    pub use crate::triad::prelude::*;
    pub use crate::types::prelude::*;
}
