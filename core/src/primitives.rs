/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::consts::*;

pub(crate) mod prelude {
    pub use super::consts::*;
}

pub mod consts {
    /// Used to describe the total number of notes considered
    pub const MODULUS: i8 = 12;
    /// A semitone is half of a tone
    pub const SEMITONE: i8 = 1;
    /// A tone is a difference of two
    pub const TONE: i8 = 2;
}

pub mod statics {
    use lazy_static::lazy_static;
    use regex::Regex;
    
    lazy_static! {
        static ref REGEX_PITCH: Regex = Regex::new("^[ABCDEFGabcdefg][b♭♯#s𝄪x]*").unwrap();
    }
}