/*
    Appellation: intervals <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{kinds::*, interval::*};

pub(crate) mod kinds;
pub(crate) mod interval;

pub(crate) mod prelude {
    pub use super::kinds::*;
    pub use super::interval::*;
}