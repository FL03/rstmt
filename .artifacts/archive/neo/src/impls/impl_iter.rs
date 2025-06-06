/*
    Appellation: impl_iter <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::triad::Triad;
use rstmt::Note;

impl IntoIterator for Triad {
    type Item = Note;

    type IntoIter = core::array::IntoIter<Self::Item, 3>;

    fn into_iter(self) -> Self::IntoIter {
        self.notes.into_iter()
    }
}

impl<'a> IntoIterator for &'a Triad {
    type Item = &'a Note;

    type IntoIter = core::slice::Iter<'a, Note>;

    fn into_iter(self) -> Self::IntoIter {
        self.notes.iter()
    }
}

impl<'a> IntoIterator for &'a mut Triad {
    type Item = &'a mut Note;

    type IntoIter = core::slice::IterMut<'a, Note>;

    fn into_iter(self) -> Self::IntoIter {
        self.notes.iter_mut()
    }
}
