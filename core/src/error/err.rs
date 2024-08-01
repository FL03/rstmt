/*
    Appellation: err <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::MusicalError;

pub trait ErrorKind: core::fmt::Debug + core::fmt::Display {}

impl<T> ErrorKind for T where T: core::fmt::Debug + core::fmt::Display {}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(transparent)]
pub struct UnknownError;

impl core::fmt::Display for UnknownError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Unknown error")
    }
}

unsafe impl Send for UnknownError {}

unsafe impl Sync for UnknownError {}

#[cfg(feature = "std")]
impl std::error::Error for UnknownError {}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]

pub struct Error<K = MusicalError> {
    pub kind: K,
    pub msg: String,
}

impl Error<UnknownError> {
    pub fn unknown(msg: impl ToString) -> Self {
        Self::new(UnknownError, msg)
    }
}

impl<K> Error<K>
where
    K: ErrorKind,
{
    pub fn new(kind: K, msg: impl ToString) -> Self {
        Self {
            kind,
            msg: msg.to_string(),
        }
    }

    pub fn kind(&self) -> &K {
        &self.kind
    }

    pub fn msg(&self) -> &str {
        &self.msg
    }
}

impl Error<MusicalError> {
    pub fn invalid_interval(msg: impl ToString) -> Self {
        Self::new(MusicalError::InvalidInterval, msg)
    }

    pub fn invalid_pitch(msg: impl ToString) -> Self {
        Self::new(MusicalError::InvalidPitch, msg)
    }
}

impl<K> core::fmt::Display for Error<K>
where
    K: ErrorKind,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}: {}", self.kind, self.msg)
    }
}

impl From<Box<dyn std::error::Error>> for Error<UnknownError> {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        Self::unknown(err.to_string())
    }
}

impl From<&str> for Error<UnknownError> {
    fn from(msg: &str) -> Self {
        Self::unknown(msg)
    }
}

impl From<String> for Error<UnknownError> {
    fn from(msg: String) -> Self {
        Self::unknown(msg)
    }
}

impl<K> From<K> for Error<K>
where
    K: ErrorKind,
{
    fn from(kind: K) -> Self {
        Self::new(kind, "")
    }
}

unsafe impl<K> Send for Error<K> where K: ErrorKind {}

unsafe impl<K> Sync for Error<K> where K: ErrorKind {}

#[cfg(feature = "std")]
impl<K> std::error::Error for Error<K> where K: ErrorKind {}
