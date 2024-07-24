/*
    Appellation: notes <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::note::*;

pub(crate) mod impls {
    pub mod impl_note_ops;
}
pub(crate) mod note;

pub(crate) mod prelude {
    pub use super::note::*;
}
