/*
    Appellation: transform <module>
    Contrib: @FL03
*/
//! this module focuses on implementing the motion planning algorithm for finding paths along
//! the surface of the tonnetz.

#[cfg(feature = "std")]
pub use self::path_finder::PathFinder;
#[cfg(feature = "tonnetz")]
pub use self::planner::MotionPlanner;
#[doc(inline)]
pub use self::types::*;

#[cfg(feature = "std")]
pub mod path_finder;
#[cfg(feature = "tonnetz")]
pub mod planner;

mod impls {
    mod impl_motion_planner;
    mod impl_path_finder;
}
mod types {
    #[doc(inline)]
    pub use self::config::*;
    #[doc(inline)]
    #[cfg(feature = "alloc")]
    pub use self::{
        cache::*, chain::*, chain_features::*, path::*, search_node::*, transform_set::*,
    };

    mod cache;
    mod chain;
    mod chain_features;
    mod config;
    mod path;
    mod search_node;
    mod transform_set;
}

pub(crate) mod prelude {
    #[cfg(feature = "alloc")]
    pub use super::path_finder::*;
    #[cfg(feature = "tonnetz")]
    pub use super::planner::*;
    pub use super::types::*;
}
