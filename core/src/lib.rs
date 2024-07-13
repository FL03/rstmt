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

pub use self::errors::{Error, Result};
pub use self::pitch::{Pitch, PitchClass, PitchTy};
pub use self::{primitives::*, types::prelude::*};

#[macro_use]
pub(crate) mod macros;
pub(crate) mod primitives;

pub mod errors;
pub mod notes;
pub mod pitch;
pub mod traits;
pub mod types;

pub mod prelude {
    pub use super::errors::prelude::*;
    pub use super::notes::prelude::*;
    pub use super::pitch::prelude::*;
    pub use super::primitives::prelude::*;
    pub use super::traits::prelude::*;
    pub use super::types::prelude::*;
}
