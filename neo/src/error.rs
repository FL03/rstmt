/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub use res::EResult;
/// A type alias for a [`Result`](core::result::Result) that uses the [`TriadError`](TriadError) type.
pub type TriadResult<T = ()> = core::result::Result<T, TriadError>;

use rstmt::{Intervals, Note, Pitch, Tuple3};

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
    InvalidPitch(Pitch),
    #[error("Invalid Interval: {0}")]
    InvalidInterval(Intervals),
    #[error("Invalid Triad: {0:?}")]
    InvalidTriad(Tuple3<Note>),
    #[error("{0}")]
    Music(#[from] rstmt::MusicalError),
    #[error("{0}")]
    Unknown(String),
}

impl TriadError {
    pub fn invalid_pitch(value: Pitch) -> Self {
        Self::InvalidPitch(value)
    }

    pub fn invalid_interval(value: Intervals) -> Self {
        Self::InvalidInterval(value)
    }

    pub fn invalid_triad(root: Note, third: Note, fifth: Note) -> Self {
        Self::InvalidTriad((root, third, fifth))
    }

    pub fn unknown(message: impl Into<String>) -> Self {
        Self::Unknown(message.into())
    }
}

impl From<Pitch> for TriadError {
    fn from(err: Pitch) -> Self {
        TriadError::InvalidPitch(err)
    }
}

impl From<(Note, Note, Note)> for TriadError {
    fn from(err: (Note, Note, Note)) -> Self {
        TriadError::InvalidTriad(err)
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
