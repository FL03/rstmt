/*
    Appellation: transform <module>
    Contrib: @FL03
*/
//! this module focuses on implementing the motion planning algorithm for finding paths along
//! the surface of the tonnetz.
#[doc(inline)]
pub use self::{
    cache::PathCache, navigator::TriadNavigator, planner::MotionPlanner, types::prelude::*,
};

pub mod cache;
pub mod navigator;
pub mod planner;

pub mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod chain;
    pub mod path;
    pub mod search_node;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::chain::*;
        #[doc(inline)]
        pub use super::path::*;
        #[doc(inline)]
        pub use super::search_node::*;
    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::navigator::TriadNavigator;
}
