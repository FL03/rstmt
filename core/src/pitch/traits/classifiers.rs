/*
    Appellation: classifiers <module>
    Created At: 2025.12.21:09:08:08
    Contrib: @FL03
*/
use crate::pitch::Accidental;

/// [`PitchRepr`] is a sealed trait used to define compatible pitch representations
/// (a.k.a pitch classes).
pub trait PitchRepr: AsRef<str> + core::fmt::Debug + core::fmt::Display {
    const IDX: isize;
    type Tag: Accidental;

    private! {}

    fn new() -> Self
    where
        Self: Sized;

    fn value(&self) -> isize {
        Self::IDX
    }
}
