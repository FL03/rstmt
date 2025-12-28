/*
    Appellation: impl_triad_ext <module>
    Created At: 2025.12.20:11:14:38
    Contrib: @FL03
*/
use crate::triad::TriadBase;

use crate::error::TriadError;
use crate::traits::{RawTriad, RawTriadMut, TriadCls};
use crate::types::{Factors, LPR};
use num_traits::{FromPrimitive, One};
use rstmt_core::{PitchMod, TryTransform};

impl<S, T, K> TryTransform<LPR> for TriadBase<S, K, T>
where
    K: TriadCls,
    S: RawTriad<Elem = T>,
    T: Copy
        + FromPrimitive
        + One
        + PitchMod<Output = T>
        + core::ops::Add<Output = T>
        + core::ops::Sub<Output = T>,
{
    type Output = TriadBase<S, K::Rel, T>;
    type Error = TriadError;

    fn try_transform(&self, transformation: LPR) -> Result<Self::Output, Self::Error> {
        LPR::try_apply(&transformation, self)
    }
}

impl<T, S, K> core::fmt::Debug for TriadBase<S, K, T>
where
    S: RawTriad<Elem = T> + core::fmt::Debug,
    K: TriadCls,
    T: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}-{:?} ({:?})", self.root(), self.class, self.chord)
    }
}

impl<T, S, K> core::fmt::Display for TriadBase<S, K, T>
where
    S: RawTriad<Elem = T> + core::fmt::Debug,
    K: TriadCls + core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{ chord: {:?}, class: {}, octave: {} }}",
            self.chord, self.class, self.octave
        )
    }
}

impl<T, S, K> AsRef<S> for TriadBase<S, K, T>
where
    S: RawTriad<Elem = T>,
    K: TriadCls,
{
    fn as_ref(&self) -> &S {
        self.chord()
    }
}

impl<T, S, K> AsMut<S> for TriadBase<S, K, T>
where
    S: RawTriad<Elem = T>,
    K: TriadCls,
{
    fn as_mut(&mut self) -> &mut S {
        self.chord_mut()
    }
}

impl<T, S, K> core::borrow::Borrow<S> for TriadBase<S, K, T>
where
    S: RawTriad<Elem = T>,
    K: TriadCls,
{
    fn borrow(&self) -> &S {
        self.chord()
    }
}

impl<T, S, K> core::borrow::BorrowMut<S> for TriadBase<S, K, T>
where
    S: RawTriad<Elem = T>,
    K: TriadCls,
{
    fn borrow_mut(&mut self) -> &mut S {
        self.chord_mut()
    }
}

impl<T, S, K> core::ops::Deref for TriadBase<S, K, T>
where
    S: RawTriad<Elem = T>,
    K: TriadCls,
{
    type Target = S;

    fn deref(&self) -> &Self::Target {
        self.chord()
    }
}

impl<T, S, K> core::ops::DerefMut for TriadBase<S, K, T>
where
    S: RawTriad<Elem = T>,
    K: TriadCls,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.chord_mut()
    }
}

impl<T, S, K> core::ops::Index<Factors> for TriadBase<S, K, T>
where
    S: RawTriad<Elem = T>,
    K: TriadCls,
{
    type Output = T;

    fn index(&self, index: Factors) -> &Self::Output {
        match index {
            Factors::Root => self.chord().root(),
            Factors::Third => self.chord().third(),
            Factors::Fifth => self.chord().fifth(),
        }
    }
}

impl<T, S, K> core::ops::IndexMut<Factors> for TriadBase<S, K, T>
where
    S: RawTriadMut<Elem = T>,
    K: TriadCls,
{
    fn index_mut(&mut self, index: Factors) -> &mut Self::Output {
        match index {
            Factors::Root => self.chord_mut().root_mut(),
            Factors::Third => self.chord_mut().third_mut(),
            Factors::Fifth => self.chord_mut().fifth_mut(),
        }
    }
}

impl<S, T, K, I> IntoIterator for TriadBase<S, K, T>
where
    S: RawTriad<Elem = T> + IntoIterator<Item = I::Item, IntoIter = I>,
    K: TriadCls,
    I: core::iter::Iterator<Item = T>,
{
    type Item = T;
    type IntoIter = I;

    fn into_iter(self) -> Self::IntoIter {
        self.chord.into_iter()
    }
}

impl<'a, S, T, K, I> IntoIterator for &'a TriadBase<S, K, T>
where
    S: RawTriad<Elem = T>,
    K: TriadCls,
    I: core::iter::Iterator<Item = &'a T>,
    &'a S: IntoIterator<Item = I::Item, IntoIter = I>,
{
    type Item = &'a T;
    type IntoIter = I;

    fn into_iter(self) -> Self::IntoIter {
        self.chord().into_iter()
    }
}

impl<'a, S, T, K, I> IntoIterator for &'a mut TriadBase<S, K, T>
where
    S: RawTriad<Elem = T>,
    K: TriadCls,
    I: core::iter::Iterator<Item = &'a T>,
    &'a mut S: IntoIterator<Item = I::Item, IntoIter = I>,
{
    type Item = &'a T;
    type IntoIter = I;

    fn into_iter(self) -> Self::IntoIter {
        self.chord_mut().into_iter()
    }
}

impl<S, T, K> PartialEq<S> for TriadBase<S, K, T>
where
    S: RawTriad<Elem = T> + PartialEq,
    K: TriadCls,
{
    fn eq(&self, other: &S) -> bool {
        self.chord() == other
    }
}

impl<S1, T1, K1, S2, T2, K2> PartialEq<TriadBase<S2, K2, T2>> for TriadBase<S1, K1, T1>
where
    S1: RawTriad<Elem = T1> + PartialEq<S2>,
    K1: TriadCls,
    S2: RawTriad<Elem = T2>,
    K2: TriadCls,
{
    fn eq(&self, other: &TriadBase<S2, K2, T2>) -> bool {
        self.chord() == other.chord()
    }
}

impl<T, S, K> From<(S, K)> for TriadBase<S, K, T>
where
    S: RawTriad<Elem = T>,
    K: TriadCls,
{
    fn from((chord, class): (S, K)) -> Self {
        Self::new(chord, class)
    }
}
