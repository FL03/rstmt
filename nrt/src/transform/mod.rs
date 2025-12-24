/*
    Appellation: transform <module>
    Contrib: @FL03
*/
//! this module focuses on implementing the motion planning algorithm for finding paths along
//! the surface of the tonnetz.

#[doc(inline)]
pub use self::prelude::*;

pub mod config;
#[cfg(feature = "std")]
pub mod navigator;
#[cfg(feature = "tonnetz")]
pub mod planner;

#[cfg(feature = "alloc")]
mod types {
    #[doc(inline)]
    pub use self::{cache::*, chain::*, path::*, search_node::*};

    mod cache;
    mod chain;
    mod path;
    mod search_node;
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::config::*;

    #[cfg(feature = "alloc")]
    #[doc(inline)]
    pub use super::navigator::*;
    #[cfg(feature = "tonnetz")]
    #[doc(inline)]
    pub use super::planner::*;
    #[cfg(feature = "alloc")]
    #[doc(inline)]
    pub use super::types::*;
}
