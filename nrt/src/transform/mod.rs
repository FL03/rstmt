/*
    Appellation: transform <module>
    Contrib: @FL03
*/
//! this module focuses on implementing the motion planning algorithm for finding paths along
//! the surface of the tonnetz.

#[cfg(feature = "tonnetz")]
#[doc(inline)]
pub use self::planner::MotionPlanner;
#[cfg(feature = "alloc")]
#[doc(inline)]
pub use self::types::prelude::*;
#[cfg(feature = "std")]
#[doc(inline)]
pub use self::{cache::PathCache, navigator::TriadNavigator};

#[cfg(feature = "std")]
pub mod cache;
#[cfg(feature = "std")]
pub mod navigator;
#[cfg(feature = "tonnetz")]
pub mod planner;

pub mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    #[cfg(feature = "std")]
    pub mod chain;
    #[cfg(feature = "std")]
    pub mod path;
    #[cfg(feature = "alloc")]
    pub mod search_node;

    pub(crate) mod prelude {
        #[cfg(feature = "std")]
        #[doc(inline)]
        pub use super::chain::*;
        #[cfg(feature = "std")]
        #[doc(inline)]
        pub use super::path::*;
        #[cfg(feature = "alloc")]
        #[doc(inline)]
        pub use super::search_node::*;
    }
}

pub(crate) mod prelude {
    #[cfg(feature = "std")]
    #[doc(inline)]
    pub use super::navigator::TriadNavigator;
    #[cfg(feature = "tonnetz")]
    #[doc(inline)]
    pub use super::planner::MotionPlanner;
}
