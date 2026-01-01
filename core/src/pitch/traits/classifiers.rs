/*
    Appellation: classifiers <module>
    Created At: 2025.12.21:09:08:08
    Contrib: @FL03
*/
use crate::pitch::Accidental;
use rstmt_traits::PitchMod;

/// [`PitchClassRepr`] is a sealed trait used to define compatible pitch representations
/// (a.k.a pitch classes).
pub trait RawPitchClass {
    type Tag: Accidental;

    private! {}

    /// returns the name of the pitch class
    fn name(&self) -> &str;
    /// returns the value associated with the pitch class
    fn index(&self) -> isize;
}

/// [`PitchClassRepr`] is a sealed trait used to define compatible pitch representations
/// (a.k.a pitch classes).
pub trait PitchClassRepr: RawPitchClass
where
    Self: AsRef<str>
        + AsRef<isize>
        + Default
        + core::fmt::Debug
        + core::fmt::Display
        + core::borrow::Borrow<isize>
        + TryFrom<isize>,
{
    const IDX: isize;

    private! {}

    fn new() -> Self
    where
        Self: Sized;

    /// returns true if the given value corresponds to this pitch class
    fn is(value: isize) -> bool
    where
        Self: Sized,
    {
        value.pmod() == Self::IDX
    }
}
