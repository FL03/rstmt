/*
    Appellation: errors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::err::*;

pub(crate) mod err;

/// A type alias for `Result<T, Error>`.
pub type Result<T = ()> = core::result::Result<T, Error>;

pub(crate) mod prelude {
    pub use super::Result;
    pub use super::err::*;
}