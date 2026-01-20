/*
    Appellation: error <module>
    Contrib: @FL03
*/
//! this module defines the [`Error`] type and provides a type alias for a
//! [`Result`](core::result::Result) with an [`Error`].
#[cfg(feature = "alloc")]
use alloc::{boxed::Box, string::String};

/// a type alias for a [`Result`](core::result::Result) with a [`Error`] type
pub type Result<T = ()> = core::result::Result<T, Error>;

/// The [`Error`] enum represents various errors that can occur in the application.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Attempted to use an invalid accidental")]
    InvalidAccidental,
    #[error("Attempted to name an invalid pitch class: {0}")]
    InvalidPitchClass(isize),
    #[error("Mismatched pitch classes, received {0} while expecting {1}")]
    MismatchedPitchClasses(isize, isize),
    #[error("Unable to parse the string into the configured type")]
    FromStrParseError,
    #[error("Unable to parse the string into the desired pitch class: {0}")]
    InvalidPitchClassParse(&'static str),
    #[error("Mismatched symbols: expected {0}, found {1}")]
    MismatchedSymbols(char, char),
    #[error("Invalid Chord")]
    InvalidChord,
    #[cfg(feature = "alloc")]
    #[error("Invalid Intervals: {0}")]
    IncompatibleIntervals(String),
    #[error("Invalid Note")]
    InvalidNote,
    // external errors
    #[error(transparent)]
    AnyError(#[from] anyhow::Error),
    #[error(transparent)]
    #[cfg(feature = "serde_json")]
    JsonError(#[from] serde_json::Error),
    // core errors
    #[error(transparent)]
    AddrParseError(#[from] core::net::AddrParseError),
    #[error("The impossible has occurred")]
    Infallible(#[from] core::convert::Infallible),
    #[error(transparent)]
    FmtError(#[from] core::fmt::Error),
    #[error(transparent)]
    Utf8Error(#[from] core::str::Utf8Error),
    // std-based errors
    #[cfg(feature = "std")]
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    // alloc-based errors
    #[cfg(feature = "alloc")]
    #[error(transparent)]
    BoxError(#[from] Box<dyn core::error::Error + Send + Sync>),
    #[cfg(feature = "alloc")]
    #[error("Unknown Error: {0}")]
    Unknown(String),
}

impl Error {
    /// creates a boxed error from the provided error
    #[cfg(feature = "alloc")]
    pub fn boxed<E>(err: E) -> Self
    where
        E: core::error::Error + Send + Sync + 'static,
    {
        Error::BoxError(Box::new(err))
    }
    /// creates a boxed error from the provided error
    #[cfg(feature = "alloc")]
    pub fn unknown<E>(err: E) -> Self
    where
        E: alloc::string::ToString,
    {
        Error::Unknown(err.to_string())
    }
}

#[cfg(feature = "alloc")]
impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::unknown(s)
    }
}

#[cfg(feature = "alloc")]
impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Unknown(s)
    }
}
