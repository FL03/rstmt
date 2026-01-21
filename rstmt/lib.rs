#![crate_name = "rstmt"]
#![crate_type = "lib"]
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
//! ## Features
//!
//! The crate is heavily feature-gated to allow for a modular approach to the various
//! components of the framework. The following features are available:
//!
//! - `nrt`: Enables the neo-Riemannian theory (NRT) module, which provides a computational
//!   representation of the NRT and its associated concepts.
//!
//! ### _Features: Dependencies_
//!
//! - `alloc`: enables the `alloc` crate, enabling the use of heap-allocated types such as
//!   `Vec`, `Box`, and `String`.
//! - `json`: enables the JSON serialization and deserialization of the framework's types
//!   using the [`serde_json`](https://docs.rs/serde_json) crate.
//! - `rayon`: enables parallelism using the [`rayon`](https://docs.rs/rayon) crate.
//! - `serde`: enables serialization and deserialization of implemented types using the
//!   [`serde`](https://docs.rs/serde) crate.
//!
//! ### _Features: Environment
//!
//! - `std`: Enables the standard library, which provides additional functionality and
//!   convenience methods for the framework.
//! - `nightly`: Enables the nightly features, which provide additional functionality and
//!   performance improvements. This feature is not recommended for production use, as it may
//!   introduce breaking changes or instability.
//!
//!
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::should_implement_trait
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(feature = "alloc", feature = "nightly"), feature(allocator_api))]
// compile time checks
#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error! { "Either the 'std' or 'alloc' feature must be enabled for this crate." }
// external crates
#[cfg(feature = "alloc")]
extern crate alloc;
// external modules
#[doc(inline)]
#[cfg(feature = "nrt")]
pub use rstmt_nrt as nrt;
// re-exports
#[doc(inline)]
pub use rstmt_core::*;
// prelude
#[doc(hidden)]
pub mod prelude {
    pub use rstmt_core::prelude::*;
    #[cfg(feature = "nrt")]
    pub use rstmt_nrt::prelude::*;
}
