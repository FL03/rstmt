/*
    Appellation: frequency <module>
    Contrib: @FL03
*/
use super::RawFrequency;

/// The [`Frequency`] type is a generic wrapper around type `T` that implements the
/// [`RawFrequency`] trait. This implementation is designed to provide a consistent interface
/// for dealing with frequencies within the crate, enabling conversion, arithmetic operations,
/// and other utilities that are common to frequency values.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Deserialize, serde_derive::Serialize),
    serde(transparent)
)]
#[repr(transparent)]
pub struct Frequency<T = f64>(pub T);

impl<T> Frequency<T>
where
    T: RawFrequency,
{
    /// returns a new instance of the [`Frequency`] wrapping the given value
    pub const fn new(index: T) -> Self {
        Frequency(index)
    }
    /// returns a new instance of the [`Frequency`] wrapping the output of the given
    /// initializer function
    pub fn create<F>(f: F) -> Self
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
        Frequency::create(T::one)
    }
    /// returns a new Frequency with the value of zero
    pub fn zero() -> Self
    where
        T: num_traits::Zero,
    {
        Frequency::create(T::zero)
    }
    /// returns a pointer to the inner value
    pub const fn as_ptr(&self) -> *const T {
        core::ptr::from_ref(self.get())
    }
    /// returns a mutable pointer to the inner value
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        core::ptr::from_mut(self.get_mut())
    }
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
    /// apply a function to the inner value and returns a new Frequency wrapping the result
    pub fn map<U, F>(self, f: F) -> Frequency<U>
    where
        F: FnOnce(T) -> U,
    {
        Frequency(f(self.value()))
    }
    /// replaces the inner value with the given one and returns the old value
    pub const fn replace(&mut self, index: T) -> T {
        core::mem::replace(self.get_mut(), index)
    }
    /// set the index to the given value
    pub fn set(&mut self, index: T) -> &mut Self {
        *self.get_mut() = index;
        self
    }
    /// swap the values of two indices
    pub const fn swap(&mut self, other: &mut Self) {
        core::mem::swap(self.get_mut(), other.get_mut());
    }
    /// consumes the current instance to create another with the given value
    pub fn with<U>(self, other: U) -> Frequency<U> {
        Frequency(other)
    }
    /// takes and returns the inner value, replacing it with the logical [`default`](Default)
    /// of the type `T`
    pub fn take(&mut self) -> T
    where
        T: Default,
    {
        core::mem::take(self.get_mut())
    }
    /// returns a new instance containing a reference to the inner value
    pub const fn view(&self) -> Frequency<&T> {
        Frequency(self.get())
    }
    /// returns a new instance containing a mutable reference to the inner value
    pub fn view_mut(&mut self) -> Frequency<&mut T> {
        Frequency(self.get_mut())
    }
}

impl<T> Default for Frequency<T>
where
    T: Default,
{
    fn default() -> Self {
        Frequency(T::default())
    }
}

contained::fmt_wrapper! {
    impl Frequency<T> {
        Binary,
        Debug,
        Display,
        LowerExp,
        LowerHex,
        Octal,
        Pointer,
        UpperExp,
        UpperHex
    }

}
