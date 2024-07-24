/*
    Appellation: impl_triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::triad::{Triad, TriadKind};
use crate::Factors;
use rstmt::Note;

impl<K> core::ops::Index<Factors> for Triad<K>
where
    K: TriadKind,
{
    type Output = Note;

    fn index(&self, index: Factors) -> &Self::Output {
        &self.notes[index as usize]
    }
}

impl<K> core::ops::IndexMut<Factors> for Triad<K>
where
    K: TriadKind,
{
    fn index_mut(&mut self, index: Factors) -> &mut Self::Output {
        &mut self.notes[index as usize]
    }
}

impl<K> core::ops::Index<core::ops::Range<Factors>> for Triad<K>
where
    K: TriadKind,
{
    type Output = [Note];

    fn index(&self, index: core::ops::Range<Factors>) -> &Self::Output {
        &self.notes[index.start as usize..index.end as usize]
    }
}

impl<K> core::ops::IndexMut<core::ops::Range<Factors>> for Triad<K>
where
    K: TriadKind,
{
    fn index_mut(&mut self, index: core::ops::Range<Factors>) -> &mut Self::Output {
        &mut self.notes[index.start as usize..index.end as usize]
    }
}
