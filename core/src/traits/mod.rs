/*
    Appellation: traits <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::notable::Notable;

#[doc(hidden)]
pub mod harmonic;
pub mod notable;
pub mod symbols;

pub(crate) mod prelude {
    pub use super::harmonic::*;
    pub use super::notable::*;
    pub use super::symbols::*;
}
