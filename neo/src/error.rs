/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(hidden)]
pub use res::EResult;
/// A type alias for a [`Result`](core::result::Result) that uses the [`TriadError`](TriadError) type.
pub type TriadResult<T = ()> = core::result::Result<T, TriadError>;

use rstmt::{Note, Pitch};

#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    strum::AsRefStr,
    strum::EnumIs,
    strum::VariantNames,
    thiserror::Error,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "PascalCase")
)]
#[repr(C)]
#[strum(serialize_all = "PascalCase")]
pub enum TriadError {
    #[error("InvalidPitch: {0}")]
    InvalidPitch(String),
    #[error(
        "Invalid Interval: the interval between {src} and {dst} is not within the given range."
    )]
    InvalidInterval { src: Note, dst: Note },
    #[error("Invalid Triad: {0:?}")]
    InvalidTriad(String),
    #[error("{0}")]
    Music(#[from] rstmt::Error),
    #[error("{0}")]
    Unknown(String),
}

impl TriadError {
    pub fn invalid_pitch(msg: impl ToString) -> Self {
        Self::InvalidPitch(msg.to_string())
    }

    pub fn invalid_interval(src: Note, dst: Note) -> Self {
        Self::InvalidInterval { src, dst }
    }

    pub fn invalid_triad(msg: impl ToString) -> Self {
        Self::InvalidTriad(msg.to_string())
    }

    pub fn unknown(msg: impl Into<String>) -> Self {
        Self::Unknown(msg.into())
    }
}

impl From<Pitch> for TriadError {
    fn from(err: Pitch) -> Self {
        TriadError::InvalidPitch(err.to_string())
    }
}

impl From<(Note, Note, Note)> for TriadError {
    fn from((r, t, f): (Note, Note, Note)) -> Self {
        TriadError::InvalidTriad(format!("({}, {}, {})", r, t, f))
    }
}

mod res {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    pub struct EResult<A, B = A> {
        pub expected: A,
        pub found: B,
    }

    impl<A, B> EResult<A, B> {
        pub fn new(expected: A, found: B) -> Self {
            Self { expected, found }
        }
    }

    impl<A, B> core::fmt::Display for EResult<A, B>
    where
        A: core::fmt::Display,
        B: core::fmt::Display,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "Expected: {}, Found: {}", self.expected, self.found)
        }
    }
}
