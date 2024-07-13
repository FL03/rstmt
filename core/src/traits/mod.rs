/*
    Appellation: traits <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::notable::Notable;

pub mod notable;

pub(crate) mod prelude {
    pub use super::notable::*;
}
