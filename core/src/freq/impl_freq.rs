/*
    appellation: impl_freq <module>
    authors: @FL03
*/
use super::Frequency;

impl<T> Frequency<T> {
    /// returns a new instance of the [`Frequency`] wrapping the given value
    pub const fn new(index: T) -> Self {
        Frequency(index)
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
    /// applies the given function to m
    pub fn map_mut<F>(&mut self, f: F) -> &mut Self
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
}

impl<T> PartialEq<T> for Frequency<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        &self.0 == other
    }
}

impl<'a, T> PartialEq<&'a T> for Frequency<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &&'a T) -> bool {
        &self.0 == *other
    }
}

impl<'a, T> PartialEq<&'a mut T> for Frequency<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &&'a mut T) -> bool {
        &self.0 == *other
    }
}

impl<T> PartialOrd<T> for Frequency<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &T) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<'a, T> PartialOrd<&'a T> for Frequency<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &&'a T) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(*other)
    }
}

impl<'a, T> PartialOrd<&'a mut T> for Frequency<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &&'a mut T) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(*other)
    }
}

impl<T> AsRef<T> for Frequency<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Frequency<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> core::borrow::Borrow<T> for Frequency<T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T> core::borrow::BorrowMut<T> for Frequency<T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> core::ops::Deref for Frequency<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> core::ops::DerefMut for Frequency<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for Frequency<T> {
    fn from(value: T) -> Self {
        Frequency(value)
    }
}
