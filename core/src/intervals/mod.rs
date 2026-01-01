/*
    appellation: intervals <module>
    authors: @FL03
*/
//! this module implements intervallic relationships in music theory
#[doc(inline)]
pub use self::{interval_base::*, traits::*, types::*};

mod interval_base;

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
    pub use self::{intervals::*, kinds::*, qualities::*, step_size::*};

    mod intervals;
    mod kinds;
    mod qualities;
    mod step_size;
}

pub(crate) mod prelude {
    pub use super::interval_base::*;
    pub use super::traits::*;
    pub use super::types::*;
}

#[cfg(test)]
mod tests {}
