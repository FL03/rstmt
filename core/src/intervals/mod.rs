/*
    appellation: intervals <module>
    authors: @FL03
*/
//! this module implements intervallic relationships in music theory
#[doc(inline)]
pub use self::{interval_base::*, types::*};

mod interval_base;

mod impls {
    mod impl_interval_base;
}

mod types {
    #[doc(inline)]
    pub use self::{kinds::*, qualities::*};

    mod kinds;
    mod qualities;
}

pub(crate) mod prelude {
    pub use super::interval_base::*;
    pub use super::types::*;
}

#[cfg(test)]
mod tests {}
