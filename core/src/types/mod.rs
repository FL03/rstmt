/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::intervals::*;

pub mod intervals;

/// A type alias for an `octave`; Musically speaking, an octave is the interval (distance) between one musical pitch and another
/// with either half or double its frequency.
pub type Octave = i8;

pub(crate) mod prelude {
    pub use super::intervals::*;
    pub use super::Octave;
}
