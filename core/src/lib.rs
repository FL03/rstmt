/*
    Appellation: triad-core <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # triad-core
//!
//! This library provides the core abstractions for the triad project.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::{
    error::{Error, Result},
    intervals::*,
    notes::Note,
    pitch::{IntoPitch, Pitch, PitchTy, Pitches},
};
#[doc(inline)]
pub use self::{ops::prelude::*, primitives::*, traits::prelude::*, types::prelude::*, utils::*};

#[macro_use]
pub(crate) mod macros;
#[macro_use]
pub(crate) mod seal;
pub(crate) mod primitives;
pub(crate) mod utils;

pub mod chords;
pub mod error;
pub mod intervals;
#[macro_use]
pub mod notes;
pub mod ops;
pub mod pitch;
pub mod tone;
pub mod traits;
pub mod types;

pub mod prelude {
    pub use super::chords::prelude::*;
    pub use super::error::prelude::*;
    pub use super::intervals::prelude::*;
    pub use super::notes::prelude::*;
    pub use super::ops::prelude::*;
    pub use super::pitch::prelude::*;
    pub use super::primitives::prelude::*;
    pub use super::tone::prelude::*;
    pub use super::traits::prelude::*;
    pub use super::types::prelude::*;
    pub use super::utils::*;
}
