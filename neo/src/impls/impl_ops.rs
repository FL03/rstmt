/*
    Appellation: impl_triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::triad::Triad;
use crate::Factors;
use core::ops::{Index, IndexMut, Range};
use rstmt::Note;

impl<K> Index<Factors> for Triad<K> {
    type Output = Note;

    fn index(&self, index: Factors) -> &Self::Output {
        &self.notes[index as usize]
    }
}

impl<K> Index<Range<Factors>> for Triad<K> {
    type Output = [Note];

    fn index(&self, index: Range<Factors>) -> &Self::Output {
        &self.notes[index.start as usize..index.end as usize]
    }
}

impl<K> IndexMut<Factors> for Triad<K> {
    fn index_mut(&mut self, index: Factors) -> &mut Self::Output {
        &mut self.notes[index as usize]
    }
}

impl<K> IndexMut<Range<Factors>> for Triad<K> {
    fn index_mut(&mut self, index: Range<Factors>) -> &mut Self::Output {
        &mut self.notes[index.start as usize..index.end as usize]
    }
}
