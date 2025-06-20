/*
    appellation: impl_octave <module>
    authors: @FL03
*/
use crate::octave::Octave;

impl<T> From<T> for Octave<T> {
    fn from(index: T) -> Self {
        Octave(index)
    }
}

impl<T> PartialEq<T> for Octave<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        self.get() == other
    }
}

impl<'a, T> PartialEq<&'a T> for Octave<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &&'a T) -> bool {
        self.get() == *other
    }
}

impl<'a, T> PartialEq<&'a mut T> for Octave<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &&'a mut T) -> bool {
        self.get() == *other
    }
}

impl<T> PartialOrd<T> for Octave<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &T) -> Option<core::cmp::Ordering> {
        self.get().partial_cmp(other)
    }
}

impl<'a, T> PartialOrd<&'a T> for Octave<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &&'a T) -> Option<core::cmp::Ordering> {
        self.get().partial_cmp(*other)
    }
}

impl<'a, T> PartialOrd<&'a mut T> for Octave<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &&'a mut T) -> Option<core::cmp::Ordering> {
        self.get().partial_cmp(*other)
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
