/*
    Appellation: traits <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{intervals::*, notable::Notable};

pub mod intervals;
pub mod notable;
#[doc(hidden)]
pub mod symbols;

pub(crate) mod prelude {
    pub use super::intervals::*;
    pub use super::notable::*;
    pub use super::symbols::*;
}

pub trait Scale {
    type Note;

    fn get(&self, note: Self::Note) -> usize;
}
