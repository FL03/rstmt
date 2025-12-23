/*
    Appellation: impl_pclass_ext <module>
    Created At: 2025.12.23:16:16:05
    Contrib: @FL03
*/
use crate::pitch::pitch_class::PitchClass;
use crate::pitch::traits::{Accidental, PitchClassRepr};

impl<N, K> AsRef<isize> for PitchClass<N, K>
where
    N: PitchClassRepr<Tag = K>,
    K: Accidental,
{
    fn as_ref(&self) -> &isize {
        self.get().as_ref()
    }
}

impl<T, K> AsRef<str> for PitchClass<T, K>
where
    T: PitchClassRepr<Tag = K>,
    K: Accidental,
{
    fn as_ref(&self) -> &str {
        self.class.as_ref()
    }
}

impl<N, K> core::borrow::Borrow<isize> for PitchClass<N, K>
where
    N: PitchClassRepr<Tag = K>,
    K: Accidental,
{
    fn borrow(&self) -> &isize {
        self.get().borrow()
    }
}

impl<N, K> core::ops::Deref for PitchClass<N, K>
where
    N: PitchClassRepr<Tag = K>,
    K: Accidental,
{
    type Target = N;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

unsafe impl<N, K> Send for PitchClass<N, K>
where
    N: PitchClassRepr<Tag = K>,
    K: Accidental,
{
}

unsafe impl<N, K> Sync for PitchClass<N, K>
where
    N: PitchClassRepr<Tag = K>,
    K: Accidental,
{
}

impl<N, K> PartialEq<isize> for PitchClass<N, K>
where
    N: PitchClassRepr<Tag = K>,
    K: Accidental,
{
    fn eq(&self, other: &isize) -> bool {
        self.get().value() == *other
    }
}

impl<N, K> PartialEq<PitchClass<N, K>> for isize
where
    N: PitchClassRepr<Tag = K>,
    K: Accidental,
{
    fn eq(&self, other: &PitchClass<N, K>) -> bool {
        *self == other.get().value()
    }
}
