/*
    Appellation: rstopo <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # triad
//!
//! This project focuses on providing concrete abstractions of musical objects discussed within the neo-Riemannian theory.
#![crate_name = "triad"]

#[doc(inline)]
pub use triad_core::*;

pub use self::triad::Triad;

pub mod triad;

pub mod prelude {
    pub use crate::triad::{ChordFactor, Triad};
    pub use triad_core::prelude::*;
}
