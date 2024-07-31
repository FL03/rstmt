/*
    Appellation: signs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub enum SymbolCount {
    Double = 2,
    Single = 1,
}
pub struct FlatSymbol(SymbolCount);

pub struct SharpSym(SymbolCount);

impl SharpSym {
    pub fn symbol(&self) -> &str {
        match self.0 {
            SymbolCount::Double => "♯♯",
            SymbolCount::Single => "♯",
        }
    }
}

impl FlatSymbol {
    pub fn symbol(&self) -> &str {
        match self.0 {
            SymbolCount::Double => "♭♭",
            SymbolCount::Single => "♭",
        }
    }
}
