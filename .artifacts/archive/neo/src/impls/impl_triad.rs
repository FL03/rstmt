/*
    Appellation: impl_triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::triad::Triad;
use rstmt::Note;

impl<K> Triad<K> {
    /// Checks if the triad is [augmented](crate::Augmented).
    pub fn is_augmented(&self) -> bool {
        self.root_to_third().is_ok_and(|i| i.is_major())
            && self.third_to_fifth().is_ok_and(|i| i.is_major())
            && self.root_to_fifth().is_ok_and(|i| i.is_augmented())
    }
    /// Checks if the triad is [diminished](crate::Diminished).
    pub fn is_diminished(&self) -> bool {
        self.root_to_third().is_ok_and(|i| i.is_minor())
            && self.third_to_fifth().is_ok_and(|i| i.is_minor())
            && self.root_to_fifth().is_ok_and(|i| i.is_diminished())
    }
    /// Checks if the triad is [major](crate::Major).
    pub fn is_major(&self) -> bool {
        self.root_to_third().is_ok_and(|i| i.is_major())
            && self.third_to_fifth().is_ok_and(|i| i.is_minor())
            && self.root_to_fifth().is_ok_and(|i| i.is_perfect())
    }
    /// Checks if the triad is [minor](crate::Minor).
    pub fn is_minor(&self) -> bool {
        self.root_to_third().is_ok_and(|i| i.is_minor())
            && self.third_to_fifth().is_ok_and(|i| i.is_major())
            && self.root_to_fifth().is_ok_and(|i| i.is_perfect())
    }
}

impl<K> core::convert::AsRef<[Note]> for Triad<K> {
    fn as_ref(&self) -> &[Note] {
        &self.notes
    }
}

impl<K> core::convert::AsMut<[Note]> for Triad<K> {
    fn as_mut(&mut self) -> &mut [Note] {
        &mut self.notes
    }
}

impl<K> core::convert::AsRef<[Note; 3]> for Triad<K> {
    fn as_ref(&self) -> &[Note; 3] {
        &self.notes
    }
}

impl<K> core::convert::AsMut<[Note; 3]> for Triad<K> {
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
