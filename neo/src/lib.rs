/*
    Appellation: rstopo <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # rstmt-neo
//!
//! This project focuses on providing concrete abstractions of musical objects discussed within the neo-Riemannian theory.
extern crate rstmt_core as rstmt;

pub use self::triad::Triad;

pub mod triad;

pub mod prelude {
    pub use crate::triad::prelude::*;
}
