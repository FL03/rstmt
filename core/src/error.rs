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
#[non_exhaustive]
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
    #[error("Mismatched symbols")]
    MismatchedSymbols,
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

#[doc(hidden)]
pub mod custom {

    #[cfg(feature = "alloc")]
    use alloc::boxed::Box;

    #[derive(
        Clone,
        Copy,
        Debug,
        Default,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        strum::AsRefStr,
        strum::Display,
        strum::EnumCount,
        strum::EnumString,
        strum::VariantNames,
    )]
    #[cfg_attr(
        feature = "serde",
        derive(serde::Deserialize, serde::Serialize),
        serde(rename_all = "snake_case")
    )]
    #[non_exhaustive]
    pub enum ErrorKind {
        InvalidAccidental,
        ChordError,
        NoteError,
        PitchError,
        InvalidPitchClass,
        MismatchedPitchClasses,
        FromStrParseError,
        InvalidPitchClassParse,
        MismatchedSymbols,
        InvalidChord,
        IncompatibleIntervals,
        InvalidNote,
        Utf8Error,
        AnyError,
        JsonError,
        AddrParseError,
        Infallible,
        FmtError,
        None,
        IOError,
        #[default]
        Unknown,
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct ErrorBase<E = Box<dyn core::error::Error + 'static>> {
        pub(crate) kind: ErrorKind,
        pub(crate) source: Option<E>,
    }

    impl<E> ErrorBase<E>
    where
        E: core::error::Error,
    {
        pub const fn new(kind: ErrorKind, source: Option<E>) -> Self {
            Self { kind, source }
        }

        pub const fn unknown(source: E) -> Self {
            Self {
                kind: ErrorKind::Unknown,
                source: Some(source),
            }
        }
        /// consumes the current error to create another of the given kind
        pub fn with_kind(self, kind: ErrorKind) -> ErrorBase<E> {
            ErrorBase {
                kind,
                source: self.source,
            }
        }
        /// consumes the current error to create another with the given message
        pub fn with_source<E2>(self, source: E2) -> ErrorBase<E2>
        where
            E2: core::error::Error,
        {
            ErrorBase {
                kind: self.kind,
                source: Some(source),
            }
        }
    }

    impl<E> core::fmt::Display for ErrorBase<E>
    where
        E: 'static + core::error::Error,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            if let Some(source) = &self.source {
                write!(f, "{}: {}", self.kind, source)
            } else {
                write!(f, "{}", self.kind)
            }
        }
    }

    impl<E> core::error::Error for ErrorBase<E>
    where
        E: 'static + core::error::Error,
    {
        fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
            self.source
                .as_ref()
                .map(|e| e as &(dyn core::error::Error + 'static))
        }
    }

    impl<T> From<Option<T>> for ErrorBase
    where
        T: core::error::Error + 'static,
    {
        fn from(opt: Option<T>) -> Self {
            match opt {
                Some(err) => Self {
                    kind: ErrorKind::Unknown,
                    source: Some(Box::new(err)),
                },
                None => Self {
                    kind: ErrorKind::None,
                    source: None,
                },
            }
        }
    }

    impl From<core::str::Utf8Error> for ErrorBase {
        fn from(err: core::str::Utf8Error) -> Self {
            Self {
                kind: ErrorKind::Utf8Error,
                #[cfg(feature = "alloc")]
                source: Some(Box::new(err)),
            }
        }
    }

    #[cfg(feature = "serde_json")]
    impl From<serde_json::Error> for ErrorBase {
        fn from(err: serde_json::Error) -> Self {
            Self {
                kind: ErrorKind::JsonError,
                source: Some(Box::new(err)),
            }
        }
    }
}
