/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::factors::{ChordFactor, Factors};

pub mod factors;

pub(crate) mod prelude {
    pub use super::factors::{ChordFactor, Factors};
}
