/*
    Appellation: errors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{err::*, kinds::*};

pub(crate) mod err;

pub mod kinds {
    pub use self::music::*;

    pub(crate) mod music;
}

pub(crate) mod prelude {
    pub use super::err::*;
    pub use super::kinds::*;
    pub use super::Result;
}

/// A type alias for `Result<T, Error>`.
pub type Result<T = ()> = core::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        let err = Error::new(
            MusicalError::InvalidInterval,
            "Invalid interval".to_string(),
        );
        assert_eq!(err.kind(), &MusicalError::InvalidInterval);
        assert_eq!(err.msg(), "Invalid interval");
    }
}
