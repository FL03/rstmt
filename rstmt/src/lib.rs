/*
    Appellation: rstmt <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # rstmt
//! 
//! [![crates.io](https://img.shields.io/crates/v/rstmt?style=for-the-badge&logo=rust)](https://crates.io/crates/rstmt)
//! [![docs.rs](https://img.shields.io/docsrs/rstmt?style=for-the-badge&logo=docs.rs)](https://docs.rs/rstmt)
//! [![GitHub License](https://img.shields.io/github/license/FL03/rstmt?style=for-the-badge&logo=github)](https://github.com/FL03/rstmt/blob/main/LICENSE)
//! 
//! ***
//!
//! Welcome to the [`rstmt`](https://docs.rs/rstmt) crate! This crate is designed to be a
//! complete computational framework for computational music theory.
//!
//! ## Goals
//!
//! The primary goal for this framework is to establish concrete computational representations
//! of various musical concepts. This is of particular importance for the [`scsys.io`](https://scsys.io)
//! ecosystem, which is drive by [`eryon`](https://docs.rs/eryon), a topological computational
//! framework inspired by the neo-Riemannian theory (NRT).
//!
//!
#![allow(clippy::module_inception, clippy::needless_doctest_main)]
#![cfg_attr(not(feature = "std"), no_std)]
#![crate_name = "rstmt"]
#! [crate_type = "lib"]

#[cfg(feature = "alloc")]
extern crate alloc;

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
