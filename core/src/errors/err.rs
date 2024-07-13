/*
    Appellation: err <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait FluoError: core::fmt::Debug + core::fmt::Display + Send + Sync + 'static {}

impl<T> FluoError for T where T: core::fmt::Debug + core::fmt::Display + Send + Sync + 'static {}

pub trait ErrorKind: Clone + core::str::FromStr + core::fmt::Debug + core::fmt::Display {}

impl<T> ErrorKind for T where T: Clone + core::str::FromStr + core::fmt::Debug + core::fmt::Display {}

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    strum::AsRefStr,
    strum::Display,
    strum::EnumIs,
    strum::EnumString,
    strum::VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "PascalCase")
)]
#[strum(serialize_all = "PascalCase")]
pub enum MusicalError {
    InvalidInterval,
    InvalidPitch,
}

#[cfg(feature = "std")]
impl std::error::Error for MusicalError {}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]

pub struct Error<K = MusicalError> {
    pub kind: K,
    pub msg: String,
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

#[cfg(feature = "std")]
impl<K> std::error::Error for Error<K> where K: ErrorKind {}

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
