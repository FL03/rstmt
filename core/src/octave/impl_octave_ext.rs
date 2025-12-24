/*
    Appellation: impl_octave_ext <module>
    Created At: 2025.12.21:08:48:17
    Contrib: @FL03
*/
use super::Octave;

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
