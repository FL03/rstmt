/*
    appellation: intervals <module>
    authors: @FL03
*/
//! this module implements intervallic relationships in music theory
#[doc(inline)]
pub use self::{interval_base::*, traits::*};

mod interval_base;

mod impls {
    mod impl_interval_base;
}

mod traits {
    #[doc(inline)]
    pub use self::qualities::*;

    mod qualities;
}

pub(crate) mod prelude {
    pub use super::interval_base::*;
    pub use super::traits::*;
}

#[cfg(test)]
mod tests {}
