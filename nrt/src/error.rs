/*
    Appellation: error <module>
    Contrib: @FL03
*/

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

/// a type alias for a [`Result`](core::result::Result) with [`TriadError`] as its error type.
pub(crate) type Result<T = ()> = core::result::Result<T, TriadError>;

/// the [`TriadError`] type enumerates the various errors that one can expect to encounter
/// within the crate.
#[derive(Debug, thiserror::Error)]
pub enum TriadError {
    #[error("Invalid triad")]
    InvalidTriad,
    #[error("Invalid Triad Class")]
    InvalidTriadClass,
    #[error(transparent)]
    #[cfg(feature = "alloc")]
    CoreError(#[from] rstmt::Error),
    #[error(transparent)]
    GraphError(#[from] rshyper::Error),
}

#[cfg(feature = "alloc")]
impl From<TriadError> for rstmt::Error {
    fn from(err: TriadError) -> Self {
        use TriadError::*;
        match err {
            CoreError(e) => e,
            _ => rstmt::Error::BoxError(Box::new(err)),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&str> for TriadError {
    fn from(err: &str) -> Self {
        rstmt::Error::from(err).into()
    }
}

#[cfg(feature = "alloc")]
impl From<String> for TriadError {
    fn from(err: String) -> Self {
        rstmt::Error::from(err).into()
    }
}
