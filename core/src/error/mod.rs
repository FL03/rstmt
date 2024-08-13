/*
    Appellation: errors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{err::*, kinds::MusicErr};

pub(crate) mod err;

#[doc(hidden)]
pub mod kinds;

pub(crate) mod prelude {
    pub use super::err::*;
    pub use super::Result;
}

/// A type alias for `Result<T, Error>`.
pub type Result<T = ()> = core::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        let err = Error::music_error("Invalid interval");
        assert_eq!(err.msg(), "Invalid interval");
    }
}
