/*
    Appellation: error <module>
    Contrib: @FL03
*/

#[cfg(feature = "alloc")]
use alloc::{boxed::Box, string::String};

/// a type alias for a [`Result`] with a [`Error`]
pub type Result<T = ()> = core::result::Result<T, Error>;

/// The [`Error`] enum represents various errors that can occur in the application.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid Chord")]
    InvalidChord,
    #[cfg(feature = "alloc")]
    #[error("Invalid Intervals: {0}")]
    InvalidIntervals(String),
    #[error("Invalid Note")]
    InvalidNote,
    #[cfg(feature = "anyhow")]
    #[error(transparent)]
    AnyError(#[from] anyhow::Error),
    #[error(transparent)]
    FmtError(#[from] core::fmt::Error),
    #[cfg(feature = "alloc")]
    #[error(transparent)]
    BoxError(#[from] Box<dyn core::error::Error + Send + Sync + 'static>),
    #[cfg(feature = "std")]
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[cfg(feature = "serde_json")]
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[cfg(feature = "alloc")]
    #[error("Unknown Error: {0}")]
    Unknown(String),
}

#[cfg(feature = "alloc")]
impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Unknown(String::from(s))
    }
}

#[cfg(feature = "alloc")]
impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Unknown(s)
    }
}
