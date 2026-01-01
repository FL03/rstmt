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
    #[error(transparent)]
    AnyError(#[from] anyhow::Error),
    #[error("The impossible has occurred")]
    Infallible(#[from] core::convert::Infallible),
    #[error(transparent)]
    FmtError(#[from] core::fmt::Error),
    #[cfg(feature = "alloc")]
    #[error(transparent)]
    BoxError(#[from] Box<dyn core::error::Error + Send + Sync>),
    #[cfg(feature = "std")]
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    #[cfg(feature = "serde_json")]
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
