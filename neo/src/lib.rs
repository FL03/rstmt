/*
    Appellation: rstopo <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # The Neo-Riemannian Theory of Music
//!
//! The neo-Riemannian theory is a loose collection of research studying chords, called triads,
//! to better understand their transformational properties. `rstmt-neo` focuses on implementing
//! the various primitives needed to work with these objects along with a plethora of 
//! analytical utilities. One of the underlying motivations for this library is to explore the
//! computational implications of the theory and to provide a foundation for further research.
//! 
//! 
extern crate rstmt_core as rstmt;

#[doc(inline)]
pub use self::{
    error::{NeoError, TriadResult},
    transform::LPR,
    triad::{
        kinds::{Augmented, Diminished, Major, Minor},
        Triad,
    },
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

pub(crate) mod impls {
    pub mod impl_iter;
    pub mod impl_ops;
    pub mod impl_triad;
    pub mod impl_variants;
}

pub mod prelude {
    pub use crate::error::{NeoError, TriadResult};
    pub use crate::tonnetz::prelude::*;
    pub use crate::transform::prelude::*;
    pub use crate::triad::prelude::*;
    pub use crate::types::prelude::*;
}
