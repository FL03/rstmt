/*
    appellation: impl_octave <module>
    authors: @FL03
*/
use crate::octave::Octave;

impl<T> Octave<T> {
    #[allow(clippy::should_implement_trait)]
    /// returns a new instance initialized using the default value of the type
    pub fn default() -> Self
    where
        T: Default,
    {
        Octave::create(T::default)
    }
    /// returns a new instance of the [`Octave`] wrapping the given value
    pub const fn new(index: T) -> Self {
        Octave(index)
    }
    /// returns a new [`Octave`] with the output of the given initializer function
    pub fn create<F>(f: F) -> Self
    where
        F: FnOnce() -> T,
    {
        Octave(f())
    }
    /// returns a new Octave with the value of one
    pub fn one() -> Self
    where
        T: num_traits::One,
    {
        Octave::create(T::one)
    }
    /// returns a new Octave with the value of zero
    pub fn zero() -> Self
    where
        T: num_traits::Zero,
    {
        Octave::create(T::zero)
    }
    /// returns a pointer to the inner value
    pub const fn as_ptr(&self) -> *const T {
        core::ptr::from_ref(&self.0)
    }
    /// returns a mutable pointer to the inner value
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        core::ptr::from_mut(&mut self.0)
    }
    /// consumes the index returning the inner value
    pub fn into_inner(self) -> T {
        self.0
    }
    #[deprecated(
        since = "0.0.5",
        note = "use `into_inner` instead; this method will be removed in the next major version."
    )]
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
    /// apply a function to the inner value and returns a new Octave wrapping the result
    pub fn map<U, F>(self, f: F) -> Octave<U>
    where
        F: FnOnce(T) -> U,
    {
        Octave(f(self.0))
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
    pub fn with<U>(self, other: U) -> Octave<U> {
        Octave(other)
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
    pub const fn view(&self) -> Octave<&T> {
        Octave(self.get())
    }
    /// returns a new instance containing a mutable reference to the inner value
    pub fn view_mut(&mut self) -> Octave<&mut T> {
        Octave(self.get_mut())
    }
}

impl<T> AsRef<T> for Octave<T> {
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<T> AsMut<T> for Octave<T> {
    fn as_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> core::borrow::Borrow<T> for Octave<T> {
    fn borrow(&self) -> &T {
        self.get()
    }
}

impl<T> core::borrow::BorrowMut<T> for Octave<T> {
    fn borrow_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> core::ops::Deref for Octave<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> core::ops::DerefMut for Octave<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}
