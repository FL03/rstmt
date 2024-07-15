/*
    Appellation: transform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::lpr::LPR;

pub(crate) mod lpr;

pub(crate) mod prelude {
    pub use super::lpr::LPR;
}
