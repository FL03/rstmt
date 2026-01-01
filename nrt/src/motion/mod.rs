/*
    Appellation: transform <module>
    Contrib: @FL03
*/
//! this module focuses on implementing the motion planning algorithm for finding paths along
//! the surface of the tonnetz.

#[doc(inline)]
pub use self::prelude::*;

#[cfg(feature = "std")]
pub mod path_finder;
#[cfg(feature = "tonnetz")]
pub mod planner;

mod impls {
    mod impl_motion_planner;
    mod impl_path_finder;
}
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
    #[cfg(feature = "alloc")]
    pub use super::path_finder::*;
    #[doc(inline)]
    #[cfg(feature = "tonnetz")]
    pub use super::planner::*;
    #[doc(inline)]
    #[cfg(feature = "alloc")]
    pub use super::types::*;
}
