/*
    Appellation: frequency <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Frequency<T>(pub T);

impl<T> Frequency<T> {
    pub fn new(period: T) -> Self {
        Self(period)
    }
    /// Returns an immutable reference to the frequency.
    pub const fn get(&self) -> &T {
        &self.0
    }
    /// Returns a mutable reference to the frequency.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
    /// Sets the frequency to the given value.
    pub fn set(&mut self, val: T) {
        self.0 = val;
    }
    /// Applies the given function to the frequency value, returning a new instance of [Frequency].
    pub fn map<U, F>(self, f: F) -> Frequency<U>
    where
        F: Fn(T) -> U,
    {
        Frequency(f(self.0))
    }
    /// Applies the funtion to a mutable frequency value, in-place.
    pub fn map_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T),
    {
        f(&mut self.0)
    }
    /// Applies the given function to the frequency value, returning a new instance of [Frequency].
    pub fn map_once<U, F>(self, f: F) -> Frequency<U>
    where
        F: FnOnce(T) -> U,
    {
        Frequency(f(self.0))
    }
    /// Returns the reciprocal of the frequency.
    pub fn recip(self) -> T
    where
        T: core::ops::Div<Output = T> + num::One,
    {
        T::one() / self.0
    }
    /// Returns the period of the frequency.
    ///
    /// Frequency: f = 1 / T therefore T = 1 / f
    pub fn period(self) -> T
    where
        T: core::ops::Div<Output = T> + num::One,
    {
        self.recip()
    }
}
