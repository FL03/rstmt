/*
    Appellation: notable <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::pitch::{Pitch, Pitches};

pub trait Notable: Copy + Sized + core::fmt::Display {
    /// Classify the pitch into a pitch class
    fn class(&self) -> Pitches {
        self.pitch().class()
    }
    /// Find the modular index of the given pitch
    fn pitch(&self) -> Pitch;
}
