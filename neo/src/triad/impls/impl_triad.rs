/*
    Appellation: impl_triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::triad::Triad;
use rstmt::Note;

impl<K> AsRef<[Note]> for Triad<K> {
    fn as_ref(&self) -> &[Note] {
        &self.notes
    }
}

impl<K> AsMut<[Note]> for Triad<K> {
    fn as_mut(&mut self) -> &mut [Note] {
        &mut self.notes
    }
}

impl<K> AsRef<[Note; 3]> for Triad<K> {
    fn as_ref(&self) -> &[Note; 3] {
        &self.notes
    }
}

impl<K> AsMut<[Note; 3]> for Triad<K> {
    fn as_mut(&mut self) -> &mut [Note; 3] {
        &mut self.notes
    }
}

impl<K> core::ops::Deref for Triad<K> {
    type Target = [Note; 3];

    fn deref(&self) -> &Self::Target {
        &self.notes
    }
}

impl<K> core::ops::DerefMut for Triad<K> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.notes
    }
}

impl<K> core::fmt::Display for Triad<K> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let (root, third, fifth) = self.as_tuple();
        write!(f, "({}, {}, {})", root, third, fifth)
    }
}
