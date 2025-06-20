/*
    appellation: impl_freq <module>
    authors: @FL03
*/
use crate::freq::Frequency;

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
