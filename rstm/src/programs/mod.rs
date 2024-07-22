/*
    Appellation: programs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{instruction::*, program::*};

pub(crate) mod instruction;
pub(crate) mod program;

pub(crate) mod prelude {
    pub use super::instruction::Instruction;
    pub use super::program::Program;
}
