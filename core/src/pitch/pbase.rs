/*
    Appellation: aspn <module>
    Contrib: @FL03
*/
use crate::PitchMod;
use crate::freq::{Frequency, RawFrequency};

/// A discrete pitch with a class and frequency.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields, rename_all = "snake_case")
)]
#[repr(C)]
pub struct Pitch<T = f32> {
    pub(crate) class: usize,
    pub(crate) freq: Frequency<T>,
}

impl<T> Pitch<T>
where
    T: RawFrequency,
{
    pub fn new(class: usize, freq: Frequency<T>) -> Self {
        Self { class, freq }
    }
    /// returns a copy to the index of the note's class
    pub const fn class(&self) -> usize {
        self.class
    }
    /// returns a mutable reference to the index of the note's class
    pub fn class_mut(&mut self) -> &mut usize {
        &mut self.class
    }
    /// returns a reference to the frequency of the pitch
    pub const fn frequency(&self) -> &Frequency<T> {
        &self.freq
    }
    /// returns a mutable reference to the current octave
    pub const fn frequency_mut(&mut self) -> &mut Frequency<T> {
        &mut self.freq
    }
    /// set the pitch class and return a mutable reference to the current instance
    pub fn set_class(&mut self, class: usize) -> &mut Self {
        self.class = class.pmod();
        self
    }
    /// set the frequency and return a mutable reference to the current instance
    pub fn set_frequency(&mut self, freq: Frequency<T>) -> &mut Self {
        self.freq = freq;
        self
    }
    /// consumes the current instance to create another with the given pitch class
    pub fn with_class(self, class: usize) -> Self {
        Self { class, ..self }
    }
    /// consumes the current instance to create another with the given frequency
    pub fn with_frequency<T2>(self, freq: Frequency<T2>) -> Pitch<T2>
    where
        T2: RawFrequency,
    {
        Pitch {
            class: self.class,
            freq,
        }
    }
}

impl<T> Default for Pitch<T>
where
    T: Default,
{
    fn default() -> Self {
        Pitch {
            class: 0,
            freq: Frequency::default(),
        }
    }
}

impl<T> core::fmt::Display for Pitch<T>
where
    T: RawFrequency,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}.{}", self.class, self.freq)
    }
}

impl<T> PartialEq<Frequency<T>> for Pitch<T>
where
    T: RawFrequency + PartialEq,
{
    fn eq(&self, other: &Frequency<T>) -> bool {
        self.frequency() == other
    }
}

impl<T> PartialEq<usize> for Pitch<T>
where
    T: RawFrequency,
{
    fn eq(&self, other: &usize) -> bool {
        self.class() == *other
    }
}

impl<T> PartialEq<Pitch<T>> for usize
where
    T: RawFrequency,
{
    fn eq(&self, other: &Pitch<T>) -> bool {
        *self == other.class()
    }
}

impl<T> PartialOrd<usize> for Pitch<T>
where
    T: RawFrequency,
{
    fn partial_cmp(&self, other: &usize) -> Option<core::cmp::Ordering> {
        self.class().partial_cmp(other)
    }
}

impl<T> PartialOrd<Pitch<T>> for usize
where
    T: RawFrequency,
{
    fn partial_cmp(&self, other: &Pitch<T>) -> Option<core::cmp::Ordering> {
        self.partial_cmp(&other.class())
    }
}
