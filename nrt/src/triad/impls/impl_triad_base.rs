/*
    appellation: impl_triad_base <module>
    authors: @FL03
*/

use crate::triad::types::{
    AugmentedTriadKind, DiminishedTriadKind, MajorTriadKind, MinorTriadKind,
};
use crate::triad::{Factors, RawStore, TriadBase, TriadKind, RawTriad};

impl<S, K> TriadBase<S, K>
where
    K: TriadKind,
    S: RawStore,
{
}

impl<S> TriadBase<S, AugmentedTriadKind>
where
    S: RawStore,
{
    /// returns a new instance of the [`TriadBase`] with the given chord and kind as an
    /// augmented triad.
    pub fn augmented(chord: S) -> Self {
        TriadBase::new(chord, AugmentedTriadKind)
    }
}

impl<S> TriadBase<S, DiminishedTriadKind>
where
    S: RawStore,
{
    /// returns a new instance of the [`TriadBase`] with the given chord and kind as a
    /// diminished triad.
    pub fn diminished(chord: S) -> Self {
        TriadBase::new(chord, DiminishedTriadKind)
    }
}

impl<S> TriadBase<S, MajorTriadKind>
where
    S: RawStore,
{
    /// returns a new instance of the [`TriadBase`] with the given chord and kind as a major
    /// triad.
    pub fn major(chord: S) -> Self {
        TriadBase::new(chord, MajorTriadKind)
    }
}

impl<S> TriadBase<S, MinorTriadKind>
where
    S: RawStore,
{
    /// returns a new instance of the [`TriadBase`] with the given chord and kind as a minor
    /// triad.
    pub fn minor(chord: S) -> Self {
        TriadBase::new(chord, MinorTriadKind)
    }
}

impl<S, K> From<(S, K)> for TriadBase<S, K>
where
    S: RawStore,
    K: TriadKind,
{
    fn from((chord, class): (S, K)) -> Self {
        Self::new(chord, class)
    }
}

impl<S, K> AsRef<S> for TriadBase<S, K>
where
    S: RawStore,
    K: TriadKind,
{
    fn as_ref(&self) -> &S {
        self.chord()
    }
}

impl<S, K> AsMut<S> for TriadBase<S, K>
where
    S: RawStore,
    K: TriadKind,
{
    fn as_mut(&mut self) -> &mut S {
        self.chord_mut()
    }
}

impl<S, K> core::borrow::Borrow<S> for TriadBase<S, K>
where
    S: RawStore,
    K: TriadKind,
{
    fn borrow(&self) -> &S {
        self.chord()
    }
}

impl<S, K> core::borrow::BorrowMut<S> for TriadBase<S, K>
where
    S: RawStore,
    K: TriadKind,
{
    fn borrow_mut(&mut self) -> &mut S {
        self.chord_mut()
    }
}

impl<T, S, K> core::ops::Deref for TriadBase<S, K>
where
    S: RawStore<Item = T> + RawTriad<T>,
    K: TriadKind,
{
    type Target = S;

    fn deref(&self) -> &Self::Target {
        self.chord()
    }
}

impl<T, S, K> core::ops::DerefMut for TriadBase<S, K>
where
    S: RawStore<Item = T> + RawTriad<T>,
    K: TriadKind,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.chord_mut()
    }
}

impl<T, S, K> core::ops::Index<Factors> for TriadBase<S, K>
where
    S: RawStore<Item = T> + RawTriad<T>,
    K: TriadKind,
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

impl<T, S, K> core::ops::IndexMut<Factors> for TriadBase<S, K>
where
    S: RawStore<Item = T> + RawTriad<T>,
    K: TriadKind,
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
    K: TriadKind,
{
    type Item = T;
    type IntoIter = core::array::IntoIter<T, 3>;

    fn into_iter(self) -> Self::IntoIter {
        self.chord.into_iter()
    }
}

