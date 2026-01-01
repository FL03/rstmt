/*
    appellation: intervals <module>
    authors: @FL03
*/
//! this module implements intervallic relationships in music theory
#[doc(inline)]
pub use self::{interval_base::*, step_size::*, traits::*, types::*};

mod interval_base;
mod step_size;

mod impls {
    mod impl_interval_base;
}

mod traits {
    #[doc(inline)]
    pub use self::{ops::*, units::*};

    mod ops;
    mod units;
}

mod types {
    #[doc(inline)]
    pub use self::{kinds::*, qualities::*};

    mod kinds;
    mod qualities;
}

pub(crate) mod prelude {
    pub use super::interval_base::*;
    pub use super::traits::*;
    pub use super::types::*;
}

#[cfg(test)]
mod tests {}
