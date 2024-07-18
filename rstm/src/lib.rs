/*
    Appellation: rstm <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # rstm
//! 
//! 

pub use self::fsm::*;

pub mod fsm;

pub mod prelude {
    pub use crate::fsm::{State, Tape, Transition, TuringMachine};
}