/*
    appellation: impl_triad_base <module>
    authors: @FL03
*/

use crate::triad::types::{
    AugmentedTriadKind, DiminishedTriadKind, MajorTriadKind, MinorTriadKind,
};
use crate::triad::{Factors, RawStore, TriadBase, TriadKind, TriadicStore};

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

impl<S, K> core::ops::Index<Factors> for TriadBase<S, K>
where
    S: TriadicStore,
    K: TriadKind,
{
    type Output = S::Item;

    fn index(&self, index: Factors) -> &Self::Output {
        match index {
            Factors::Root => self.chord().root(),
            Factors::Third => self.chord().third(),
            Factors::Fifth => self.chord().fifth(),
        }
    }
}

impl<S, K> core::ops::IndexMut<Factors> for TriadBase<S, K>
where
    S: TriadicStore,
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
