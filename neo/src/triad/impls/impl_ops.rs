/*
    Appellation: impl_triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::transform::{Transform, LPR};
use crate::triad::{Triad, TriadCls, TriadKind, Triads};
use crate::Factors;
use rstmt::Note;

impl<K> Transform<LPR> for Triad<K>
where
    K: TriadCls,
{
    type Output = Result<Triad<Triads>, crate::TriadError>;

    fn transform(self, lpr: LPR) -> Self::Output {
        crate::transform::utils::_transform(self, lpr)
    }
}

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
