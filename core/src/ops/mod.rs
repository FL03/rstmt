/*
    Appellation: ops <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::prelude::*;

pub mod absmod;
pub mod arith;
pub mod distance;

pub(crate) mod prelude {
    pub use super::absmod::*;
    pub use super::arith::*;
    pub use super::distance::*;
}
