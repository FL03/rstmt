/*
    Appellation: classifiers <module>
    Created At: 2025.12.21:09:08:08
    Contrib: @FL03
*/
use crate::pitch::RawAccidental;
use rstmt_traits::PitchMod;

/// The [`RawPitchClass`] is a sealed trait used to define raw pitch class types.
pub trait RawPitchClass
where
    Self: Send
        + Sync
        + AsRef<str>
        + AsRef<isize>
        + PartialEq<isize>
        + PartialEq<str>
        + core::borrow::Borrow<isize>
        + core::fmt::Debug
        + core::fmt::Display,
{
    type Tag: RawAccidental;

    private! {}

    fn new() -> Self
    where
        Self: Sized;
    /// returns the name of the pitch class
    fn name(&self) -> &str;
    /// returns the value associated with the pitch class
    fn index(&self) -> isize;
}

/// [`PitchClassRepr`] extends [`RawPitchClass`], providing various initialization routines,
/// defaults, and other methods useful for pitch class representations.
pub trait PitchClassRepr: RawPitchClass
where
    Self: Default + core::str::FromStr<Err = crate::error::Error> + TryFrom<isize>,
{
    const IDX: isize;

    /// returns true if the given value corresponds to this pitch class
    fn is(value: isize) -> bool
    where
        Self: Sized,
    {
        value.pmod() == Self::IDX
    }
}
