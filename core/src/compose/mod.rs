/*
    Appellation: comp <module>
    Created At: 2025.12.29:16:45:11
    Contrib: @FL03
*/
//! composition related implementations, primitives, and utilities for creating music with the
//! framework.
//!
#[doc(inline)]
pub use self::scale::*;

pub mod scale;

mod impls {
    mod impl_scale;
    mod impl_scale_repr;
}

#[doc(hidden)]
pub(crate) mod prelude {
    pub use super::scale::*;
}
