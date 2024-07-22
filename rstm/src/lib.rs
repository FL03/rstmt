/*
    Appellation: rstm <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # rstm
//!
//!
#[doc(inline)]
pub use self::{error::*, types::prelude::*};

pub mod error;
pub mod fsm;
pub mod programs;
pub mod state;
pub mod turing;
pub mod types;

pub mod prelude {
    pub use crate::error::FsmError;
    pub use crate::programs::prelude::*;
    pub use crate::state::prelude::*;
    pub use crate::turing::prelude::*;
    pub use crate::types::prelude::*;
}
