/*
    Appellation: signs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::Error;

pub struct Pitcher<T> {
    pub class: crate::pitch::Natural,
    pub pitch: T,
    pub sign: Option<Sign>,
}

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
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]

pub enum SymbolCount {
    Double = 2,
    #[default]
    Single = 1,
}
pub struct FlatSymbol(pub SymbolCount);

pub struct SharpSym(pub SymbolCount);

pub enum Sign {
    Flat(FlatSymbol),
    Sharp(SharpSym),
}

impl SharpSym {
    pub const REPR: char = '#';
    pub const SYMBOLS: [&'static str; 6] = ["♯", "♯♯", "s", "S", "ss", "SS"];

    pub fn symbol(&self) -> &str {
        match self.0 {
            SymbolCount::Double => Self::SYMBOLS[1],
            SymbolCount::Single => Self::SYMBOLS[0],
        }
    }
}

impl FlatSymbol {
    pub const REPR: char = '♭';
    pub const SYMBOLS: [&'static str; 8] = ["♭", "♭♭", "b", "bb", "f", "ff", "F", "FF"];

    pub fn symbol(&self) -> &str {
        match self.0 {
            SymbolCount::Double => Self::SYMBOLS[1],
            SymbolCount::Single => Self::SYMBOLS[0],
        }
    }
}

impl Sign {
    pub fn symbol(&self) -> &str {
        match self {
            Sign::Flat(flat) => flat.symbol(),
            Sign::Sharp(sharp) => sharp.symbol(),
        }
    }
}

impl core::str::FromStr for Sign {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(flat) = s.parse::<FlatSymbol>() {
            return Ok(Self::Flat(flat));
        } else if let Ok(sharp) = s.parse::<SharpSym>() {
            return Ok(Self::Sharp(sharp));
        } else {
            Err(Error::parse_error(
                "No accepted representation of the sign was found.",
            ))
        }
    }
}

impl core::str::FromStr for SharpSym {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "♯" | "s" | "S" => Ok(Self(SymbolCount::Single)),
            "♯♯" | "ss" | "SS" => Ok(Self(SymbolCount::Double)),
            _ => Err(Error::parse_error(
                "No accepted representation of the sharp symbol was found.",
            )),
        }
    }
}

impl core::str::FromStr for FlatSymbol {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "♭" | "b" | "f" | "F" => Ok(Self(SymbolCount::Single)),
            "♭♭" | "bb" | "ff" | "FF" => Ok(Self(SymbolCount::Double)),
            _ => Err(Error::parse_error(
                "No accepted representation of the flat symbol was found.",
            )),
        }
    }
}
