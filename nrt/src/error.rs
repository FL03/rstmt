/*
    Appellation: error <module>
    Contrib: @FL03
*/

#[cfg(feature = "alloc")]
use alloc::{boxed::Box, string::String};

/// a type alias for a [Result] with a [MusicError]
pub(crate) type Result<T = ()> = core::result::Result<T, TriadError>;

#[derive(Debug, thiserror::Error)]
pub enum TriadError {
    #[error("Invalid triad")]
    InvalidTriad,
    #[error("Invalid Triad Class")]
    InvalidTriadClass,
    #[cfg(feature = "anyhow")]
    #[error(transparent)]
    AnyError(#[from] anyhow::Error),
    #[error(transparent)]
    FmtError(#[from] core::fmt::Error),
    #[error(transparent)]
    CoreError(#[from] rstmt::Error),
    #[cfg(feature = "alloc")]
    #[error(transparent)]
    BoxError(#[from] Box<dyn core::error::Error + Send + Sync + 'static>),
    #[cfg(feature = "std")]
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[cfg(feature = "serde_json")]
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error(transparent)]
    GraphError(#[from] rshyper::Error),
    #[cfg(feature = "alloc")]
    #[error("Unknown Error: {0}")]
    Unknown(String),
}

#[cfg(feature = "alloc")]
impl From<&str> for TriadError {
    fn from(s: &str) -> Self {
        TriadError::Unknown(String::from(s))
    }
}

#[cfg(feature = "alloc")]
impl From<String> for TriadError {
    fn from(s: String) -> Self {
        TriadError::Unknown(s)
    }
}
