/*
    Appellation: transform <module>
    Contrib: @FL03
*/
//! this module focuses on implementing the motion planning algorithm for finding paths along
//! the surface of the tonnetz.

#[doc(inline)]
pub use self::config::*;
#[cfg(feature = "alloc")]
pub use self::path_finder::PathFinder;
#[cfg(feature = "tonnetz")]
pub use self::planner::MotionPlanner;
#[cfg(feature = "alloc")]
#[doc(inline)]
pub use self::types::*;

#[cfg(feature = "std")]
pub mod path_finder;
#[cfg(feature = "tonnetz")]
pub mod planner;

mod config {
    #[doc(inline)]
    pub use self::{motion_config::*, path_finder_config::*};

    mod motion_config;
    mod path_finder_config;
}

mod impls {
    mod impl_motion_planner;
    mod impl_path_finder;
}

#[cfg(feature = "alloc")]
mod types {
    #[doc(inline)]
    pub use self::{cache::*, chain::*, chain_features::*, path::*, search_node::*, visited::*};

    mod cache;
    mod chain;
    mod chain_features;
    mod path;
    mod search_node;
    mod visited;
}

pub(crate) mod prelude {
    pub use super::config::*;
    #[cfg(feature = "alloc")]
    pub use super::path_finder::*;
    #[cfg(feature = "tonnetz")]
    pub use super::planner::*;
    #[cfg(feature = "alloc")]
    pub use super::types::*;
}
