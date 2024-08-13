/*
    Appellation: err <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

///
#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, strum::EnumIs, strum::VariantNames,
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]

pub enum Error {
    InvalidInterval(String),
    MusicError(String),
    ParseError(String),
    Unknown(String),
}

impl Error {
    pub fn invalid_interval(msg: impl ToString) -> Self {
        Self::InvalidInterval(msg.to_string())
    }

    pub fn music_error(msg: impl ToString) -> Self {
        Self::MusicError(msg.to_string())
    }

    pub fn parse_error(msg: impl ToString) -> Self {
        Self::ParseError(msg.to_string())
    }

    pub fn unknown_error(msg: impl ToString) -> Self {
        Self::Unknown(msg.to_string())
    }

    pub fn kind(&self) -> &str {
        match self {
            Self::InvalidInterval(_) => "InvalidInterval",
            Self::MusicError(_) => "MusicError",
            Self::ParseError(_) => "ParseError",
            Self::Unknown(_) => "Unknown",
        }
    }

    pub fn msg(&self) -> &str {
        match self {
            Self::InvalidInterval(msg) => msg,
            Self::MusicError(msg) => msg,
            Self::ParseError(msg) => msg,
            Self::Unknown(msg) => msg,
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}: {}", self.kind(), self.msg())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
