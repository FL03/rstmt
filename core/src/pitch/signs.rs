/*
    Appellation: signs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, strum::AsRefStr, strum::Display)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]

pub enum SymbolCount {
    Double = 2,
    #[default]
    Single = 1,
}
pub struct FlatSymbol {
    pub(crate) count: SymbolCount,
}

pub struct SharpSym {
    pub(crate) count: SymbolCount,
}

impl SharpSym {
    pub const REPR: char = '#';
    pub const SYMBOLS: [&'static str; 2] = ["♯", "♯♯"];

    pub fn symbol(&self) -> &str {
        match self.count {
            SymbolCount::Double => "♯♯",
            SymbolCount::Single => "♯",
        }
    }
}

impl FlatSymbol {
    pub const REPR: char = '♭';
    pub const SYMBOLS: [&'static str; 2] = ["♭", "♭♭"];

    pub fn symbol(&self) -> &str {
        match self.count {
            SymbolCount::Double => "♭♭",
            SymbolCount::Single => "♭",
        }
    }
}
