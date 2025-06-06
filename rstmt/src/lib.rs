/*
    Appellation: rstmt <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # rstmt
//!
//! Welcome to the [`rstmt`](https://docs.rs/rstmt) crate! This crate is designed to be a
//! complete computational framework for computational music theory.
#![crate_name = "rstmt"]

#[doc(inline)]
pub use rstmt_core::*;
/// this module focuses on implementing the basis of the neo-riemannian theory (NRT)
#[doc(inline)]
#[cfg(feature = "nrt")]
pub use rstmt_nrt as nrt;

pub mod prelude {
    pub use rstmt_core::prelude::*;
    #[cfg(feature = "nrt")]
    pub use rstmt_nrt::prelude::*;
}
