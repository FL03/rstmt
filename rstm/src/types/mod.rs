/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{
    direction::Direction,
    parts::{Head, Tail},
    tape::Tape,
};

pub mod agent;
pub mod direction;
pub mod parts;
pub mod tape;

pub(crate) mod prelude {
    pub use super::direction::Direction;
    pub use super::parts::{Head, Tail};
    pub use super::tape::Tape;
}

pub type TStore<T> = std::collections::HashMap<Head<T>, Tail<T>>;
