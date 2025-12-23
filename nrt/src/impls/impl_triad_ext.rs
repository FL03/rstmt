/*
    Appellation: impl_triad_ext <module>
    Created At: 2025.12.20:11:14:38
    Contrib: @FL03
*/
use crate::triad::TriadBase;

use crate::traits::{RawTriad, RawTriadStore, RawTriadStoreMut, TriadCls};
use crate::types::Factors;

impl<T, K> RawTriad<T> for TriadBase<[T; 3], K, T>
where
    K: TriadCls,
{
    type Store<U> = [U; 3];

    seal!();

    fn store(&self) -> &Self::Store<T> {
        self.chord()
    }

    fn store_mut(&mut self) -> &mut Self::Store<T> {
        self.chord_mut()
    }
}

impl<T, S, K> core::fmt::Display for TriadBase<S, K, T>
where
    S: RawTriadStore<Elem = T> + core::fmt::Debug,
    K: TriadCls + core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{{ chord: {:?}, class: {} }}", self.chord, self.class)
    }
}

impl<T, S, K> From<(S, K)> for TriadBase<S, K, T>
where
    S: RawTriadStore<Elem = T>,
    K: TriadCls,
{
    fn from((chord, class): (S, K)) -> Self {
        Self::new(chord, class)
    }
}

impl<T, S, K> AsRef<S> for TriadBase<S, K, T>
where
    S: RawTriadStore<Elem = T>,
    K: TriadCls,
{
    fn as_ref(&self) -> &S {
        self.chord()
    }
}

impl<T, S, K> AsMut<S> for TriadBase<S, K, T>
where
    S: RawTriadStore<Elem = T>,
    K: TriadCls,
{
    fn as_mut(&mut self) -> &mut S {
        self.chord_mut()
    }
}

impl<T, S, K> core::borrow::Borrow<S> for TriadBase<S, K, T>
where
    S: RawTriadStore<Elem = T>,
    K: TriadCls,
{
    fn borrow(&self) -> &S {
        self.chord()
    }
}

impl<T, S, K> core::borrow::BorrowMut<S> for TriadBase<S, K, T>
where
    S: RawTriadStore<Elem = T>,
    K: TriadCls,
{
    fn borrow_mut(&mut self) -> &mut S {
        self.chord_mut()
    }
}

impl<T, S, K> core::ops::Deref for TriadBase<S, K, T>
where
    S: RawTriadStore<Elem = T> + RawTriad<T>,
    K: TriadCls,
{
    type Target = S;

    fn deref(&self) -> &Self::Target {
        self.chord()
    }
}

impl<T, S, K> core::ops::DerefMut for TriadBase<S, K, T>
where
    S: RawTriadStore<Elem = T> + RawTriad<T>,
    K: TriadCls,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.chord_mut()
    }
}

impl<T, S, K> core::ops::Index<Factors> for TriadBase<S, K, T>
where
    S: RawTriadStore<Elem = T>,
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
    S: RawTriadStoreMut<Elem = T>,
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

impl<T, K> IntoIterator for TriadBase<[T; 3], K>
where
    K: TriadCls,
{
    type Item = T;
    type IntoIter = core::array::IntoIter<T, 3>;

    fn into_iter(self) -> Self::IntoIter {
        self.chord.into_iter()
    }
}

impl<'a, T, K> IntoIterator for &'a TriadBase<[T; 3], K>
where
    K: TriadCls,
{
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.chord().iter()
    }
}

impl<'a, T, K> IntoIterator for &'a mut TriadBase<[T; 3], K>
where
    K: TriadCls,
{
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.chord_mut().iter_mut()
    }
}
