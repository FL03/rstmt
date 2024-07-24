/*
    Appellation: rstopo <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # rstmt-neo
//!
//! This project focuses on providing concrete abstractions of musical objects discussed within the neo-Riemannian theory.
extern crate rstmt_core as rstmt;

#[doc(inline)]
pub use self::{
    error::{TriadError, TriadResult},
    triad::{Triad, TriadBuilder},
    types::*,
    utils::*,
};

#[macro_use]
pub(crate) mod macros;
#[macro_use]
pub(crate) mod seal;
pub(crate) mod utils;

pub mod error;
pub mod tonnetz;
pub mod transform;
pub mod triad;
pub mod types;

#[doc(hidden)]
mod impls {}

pub mod prelude {
    pub use crate::error::{TriadError, TriadResult};
    pub use crate::tonnetz::prelude::*;
    pub use crate::transform::prelude::*;
    pub use crate::triad::prelude::*;
    pub use crate::types::prelude::*;
}
