/*
    Appellation: impl_pitch <module>
    Created At: 2025.12.20:09:09:08
    Contrib: @FL03
*/
use crate::pitch::{Pitch, RawPitch};

impl<T> Pitch<T>
where
    T: RawPitch,
{
    /// returns a new instance of the [`Pitch`] wrapping the given value
    pub const fn new(value: T) -> Self {
        Pitch(value)
    }
    /// initialize a new instance of the pitch using the result of the given function
    pub fn init<F>(f: F) -> Self
    where
        F: FnOnce() -> T,
    {
        Pitch::new(f())
    }
    /// returns a new Pitch with the value of one
    pub fn one() -> Self
    where
        T: num_traits::One,
    {
        Pitch::init(T::one)
    }
    /// returns a new Pitch with the value of zero
    pub fn zero() -> Self
    where
        T: num_traits::Zero,
    {
        Pitch::init(T::zero)
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
    /// apply a function to the inner value and returns a new Pitch wrapping the result
    pub fn map<U, F>(self, f: F) -> Pitch<U>
    where
        F: FnOnce(T) -> U,
    {
        Pitch(f(self.value()))
    }
    #[inline]
    /// applies the function onto a mutable reference of the inner value
    pub fn apply_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T),
    {
        f(self.get_mut())
    }
    /// [`replace`](core::mem::replace) the pitch with the given value, returning the previous state.
    pub const fn replace(&mut self, index: T) -> T {
        core::mem::replace(self.get_mut(), index)
    }
    #[inline]
    /// set the index to the given value
    pub fn set(&mut self, index: T) {
        *self.get_mut() = index;
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
    /// returns a new instance containing a reference to the inner value
    pub const fn view(&self) -> Pitch<&T> {
        Pitch(self.get())
    }
    /// returns a new instance containing a mutable reference to the inner value
    pub const fn view_mut(&mut self) -> Pitch<&mut T> {
        Pitch(self.get_mut())
    }
    #[inline]
    /// consumes the current instance to create another with the given value
    pub fn with<U>(self, value: U) -> Pitch<U> {
        Pitch(value)
    }
}
