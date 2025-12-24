/*
    Appellation: transform <module>
    Contrib: @FL03
*/
//! this module focuses on implementing the motion planning algorithm for finding paths along
//! the surface of the tonnetz.

#[doc(inline)]
pub use self::prelude::*;

#[cfg(feature = "std")]
pub mod cache;
pub mod config;
#[cfg(feature = "std")]
pub mod navigator;
#[cfg(feature = "tonnetz")]
pub mod planner;

mod types {
    #[cfg(feature = "alloc")]
    #[doc(inline)]
    pub use self::search_node::*;
    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use self::{chain::*, path::*};

    #[cfg(feature = "std")]
    mod chain;
    #[cfg(feature = "std")]
    mod path;
    #[cfg(feature = "alloc")]
    mod search_node;
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::config::*;

    #[cfg(feature = "std")]
    #[doc(inline)]
    pub use super::cache::*;
    #[cfg(feature = "std")]
    #[doc(inline)]
    pub use super::navigator::*;
    #[cfg(feature = "tonnetz")]
    #[doc(inline)]
    pub use super::planner::*;
    #[cfg(feature = "alloc")]
    #[doc(inline)]
    pub use super::types::*;
}
