/*
    appellation: intervals <module>
    authors: @FL03
*/
//! this module implements intervallic relationships in music theory
#[doc(inline)]
pub use self::{interval::IntervalBase, types::prelude::*};

pub mod dyad;
pub mod interval;

pub mod types {
    //! this module imimplements various types and other primitives used throughout the library
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod qualities;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::qualities::*;
    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::types::prelude::*;
}
