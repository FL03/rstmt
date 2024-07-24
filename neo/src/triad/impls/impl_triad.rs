/*
    Appellation: impl_triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::triad::{Triad, Triads, TriadKind};
use rstmt::Note;

impl<K: TriadKind> Triad<K> {
    /// Returns the [class](Triads) of the triad
    pub fn class(&self) -> Triads {
        K::class()
    }
}

impl<K: TriadKind> AsRef<[Note]> for Triad<K> {
    fn as_ref(&self) -> &[Note] {
        &self.notes
    }
}

impl<K: TriadKind> AsRef<[Note; 3]> for Triad<K> {
    fn as_ref(&self) -> &[Note; 3] {
        &self.notes
    }
}

impl<K: TriadKind> AsMut<[Note]> for Triad<K> {
    fn as_mut(&mut self) -> &mut [Note] {
        &mut self.notes
    }
}

impl<K: TriadKind> AsMut<[Note; 3]> for Triad<K> {
    fn as_mut(&mut self) -> &mut [Note; 3] {
        &mut self.notes
    }
}

impl<K> core::fmt::Display for Triad<K>
where
    K: TriadKind,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let (root, third, fifth) = self.as_tuple();
        write!(f, "({}, {}, {})", root, third, fifth)
    }
}
