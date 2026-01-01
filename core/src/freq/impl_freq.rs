/*
    appellation: impl_freq <module>
    authors: @FL03
*/
use super::{Frequency, RawFrequency};
use crate::consts::A4_FREQUENCY;
use crate::pitch::{Accidental, PitchClass, RawPitchClass};
use crate::utils::{classify_freq_with_scale, compute_freq_of_pitch};
use num_traits::{Float, FromPrimitive, ToPrimitive};
use rstmt_traits::ClassifyBy;

impl<T> Frequency<T>
where
    T: RawFrequency,
{
    /// returns a new instance of the [`Frequency`] wrapping the given value
    pub const fn new(index: T) -> Self {
        Frequency(index)
    }
    /// calculate the frequency (in hertz) of a given pitch class, using the formula:
    ///
    /// ```math
    /// F=\gamma\cdot{2^\frac{n}{12}}
    /// ```
    pub fn from_class_with_scale<N>(note: N, root: T) -> Self
    where
        N: ToPrimitive,
        T: Float + FromPrimitive,
    {
        let class = note.to_isize().unwrap();
        Self(compute_freq_of_pitch(class, root))
    }
    /// a shorthand method for creating a new frequency from the given pitch class using A4 as
    /// the base frequency
    pub fn from_class_on_a4<N>(note: N) -> Self
    where
        N: ToPrimitive,
        T: Float + FromPrimitive,
    {
        let root = <T>::from_f64(A4_FREQUENCY).unwrap();
        Self::from_class_with_scale(note, root)
    }
    /// a method for directly converting an instance of the [`PitchClass`] into a frequency
    pub fn from_pitch_class<P, K>(class: PitchClass<P, K>, root: T) -> Self
    where
        P: RawPitchClass<Tag = K>,
        K: Accidental,
        T: RawFrequency + Float + FromPrimitive,
    {
        let semitones = class.get().index();
        Self::from_class_with_scale(semitones, root)
    }
    /// initializes a new frequency by capturing the result of the given function
    pub fn init<F>(f: F) -> Self
    where
        F: FnOnce() -> T,
    {
        Frequency(f())
    }
    /// returns a new Frequency with the value of one
    pub fn one() -> Self
    where
        T: num_traits::One,
    {
        Frequency::init(T::one)
    }
    /// returns a new Frequency with the value of zero
    pub fn zero() -> Self
    where
        T: num_traits::Zero,
    {
        Frequency::init(T::zero)
    }
    /// returns a pointer to the inner value
    pub const fn as_ptr(&self) -> *const T {
        core::ptr::from_ref(self.get())
    }
    /// returns a mutable pointer to the inner value
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        core::ptr::from_mut(self.get_mut())
    }
    #[inline]
    /// consumes the index returning the inner value
    pub fn value(self) -> T {
        self.0
    }
    /// returns an immutable reference to the inner value
    pub const fn get(&self) -> &T {
        &self.0
    }
    /// returns a mutable reference to the inner value
    pub const fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
    #[inline]
    /// apply a function to the inner value and returns a new Frequency wrapping the result
    pub fn map<U, F>(self, f: F) -> Frequency<U>
    where
        F: FnOnce(T) -> U,
    {
        Frequency(f(self.value()))
    }
    #[inline]
    /// apply a function to a reference of the current frequency, capturing the result in a
    /// new instance
    pub fn apply<U, F>(&self, mut f: F) -> Frequency<U>
    where
        F: FnMut(&T) -> U,
    {
        Frequency(f(self.get()))
    }
    /// applies the given function to m
    pub fn apply_inplace<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut T),
    {
        f(self.get_mut());
        self
    }
    /// replaces the inner value with the given one and returns the old value
    pub const fn replace(&mut self, index: T) -> T {
        core::mem::replace(self.get_mut(), index)
    }
    #[inline]
    /// set the index to the given value
    pub fn set(&mut self, index: T) -> &mut Self {
        *self.get_mut() = index;
        self
    }
    /// swap the values of two indices
    pub const fn swap(&mut self, other: &mut Self) {
        core::mem::swap(self.get_mut(), other.get_mut());
    }
    #[inline]
    /// takes and returns the inner value, replacing it with the logical [`default`](Default)
    /// of the type `T`
    pub fn take(&mut self) -> T
    where
        T: Default,
    {
        core::mem::take(self.get_mut())
    }
    #[inline]
    /// consumes the current instance to create another with the given value
    pub fn with<U>(self, other: U) -> Frequency<U> {
        Frequency(other)
    }
    /// returns a new instance containing a reference to the inner value
    pub const fn view(&self) -> Frequency<&T> {
        Frequency(self.get())
    }
    /// returns a new instance containing a mutable reference to the inner value
    pub const fn view_mut(&mut self) -> Frequency<&mut T> {
        Frequency(self.get_mut())
    }
    /// Compute the pitch class of a frequency (in hertz), using the formula:
    ///
    /// ```math
    /// n = 12\cdot\log_2(\frac{F}{\gamma})
    /// ```
    pub fn classify_by<Z>(&self, base: T) -> Z
    where
        Self: ClassifyBy<T, Output = Z>,
    {
        ClassifyBy::classify_by(self, base)
    }
}

impl<T> ClassifyBy<T> for Frequency<T>
where
    T: RawFrequency + Float + FromPrimitive,
{
    type Output = isize;

    fn classify_by(&self, base: T) -> Self::Output {
        classify_freq_with_scale(self.value(), base).expect("failed to classify frequency")
    }
}
