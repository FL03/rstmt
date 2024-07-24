/*
    Appellation: rstmt <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # rstmt
//!
//! This crate focuses on implementing various musical objects and operations.
#![crate_name = "rstmt"]

#[doc(inline)]
pub use rstmt_core::*;
#[doc(inline)]
#[cfg(feature = "neo")]
pub use rstmt_neo as neo;

#[cfg(feature = "neo")]
pub mod space;

pub mod prelude {
    #[cfg(feature = "neo")]
    pub use super::space::prelude::*;
    pub use rstmt_core::prelude::*;
    #[cfg(feature = "neo")]
    pub use rstmt_neo::prelude::*;
}
