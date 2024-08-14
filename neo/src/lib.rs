/*
    Appellation: rstopo <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # The Neo-Riemannian Theory of Music
//!
//! The neo-Riemannian theory is a loose collection of research and ideas focused on the triad
//! and its transformative nature.
//! 
extern crate rstmt_core as rstmt;

#[doc(inline)]
pub use self::{
    error::{TriadError, TriadResult},
    triad::{Triad, TriadBuilder},
    types::*,
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
