/*
    appellation: intervals <module>
    authors: @FL03
*/
//! this module implements intervallic relationships in music theory
#[doc(inline)]
pub use self::{interval_base::IntervalBase, traits::*};

mod interval_base;

mod traits {
    #[doc(inline)]
    pub use self::qualities::*;

    mod qualities;
}

pub(crate) mod prelude {
    pub use super::interval_base::IntervalBase;
    pub use super::traits::*;
}
