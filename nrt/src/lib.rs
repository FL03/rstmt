/*
    Appellation: rstmt-nrt <library>
    Contrib: @FL03
*/
//! # rstmt-nrt
//!
//! This crate works to establish a solid foundation for working with the neo-riemannian theory
//!
#![allow(clippy::module_inception)]
#![cfg_attr(not(feature = "std"), no_std)]
#![crate_type = "lib"]

#[cfg(feature = "alloc")]
extern crate alloc;

/// re-declare the external `rstmt_core` crate for convenience
extern crate rstmt_core as rstmt;

#[doc(inline)]
pub use self::{
    error::*,
    triad::{Triad, Triads},
    types::prelude::*,
};

#[cfg(feature = "std")]
pub use self::transform::TriadNavigator;

#[cfg(feature = "tonnetz")]
#[doc(inline)]
pub use self::{transform::MotionPlanner, tonnetz::Tonnetz};

#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
}

pub mod error;
#[cfg(feature = "tonnetz")]
pub mod tonnetz;
pub mod transform;
pub mod triad;

#[allow(unused_imports)]
pub mod traits {
    #[doc(inline)]
    pub use self::prelude::*;

    pub(crate) mod prelude {}
}

pub mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod lpr;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::lpr::*;
    }
}

pub mod prelude {
    #[doc(no_inline)]
    pub use crate::error::*;
    #[cfg(feature = "tonnetz")]
    #[doc(no_inline)]
    pub use crate::tonnetz::Tonnetz;
    #[cfg(feature = "alloc")]
    #[doc(no_inline)]
    pub use crate::transform::prelude::*;
    #[doc(no_inline)]
    pub use crate::triad::prelude::*;
    #[doc(no_inline)]
    pub use crate::types::prelude::*;
}
