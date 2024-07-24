/*
    Appellation: impl_triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use rstmt::Note;
use strum::IntoEnumIterator;

use crate::triad::{Augmented, Diminished, Major, Minor, Triad, TriadKind, Triads};

impl<K: TriadKind> Triad<K> {
    /// Checks if the triad is [augmented](Augmented).
    pub fn is_augmented(&self) -> bool {
        <K>::is_augmented()
    }
    /// Checks if the triad is [diminished](Diminished).
    pub fn is_diminished(&self) -> bool {
        <K>::is_diminished()
    }
    /// Checks if the triad is [major](Major).
    pub fn is_major(&self) -> bool {
        <K>::is_major()
    }
    /// Checks if the triad is [minor](Minor).
    pub fn is_minor(&self) -> bool {
        <K>::is_minor()
    }
}

impl Triad<Triads> {
    /// Consumes the instance to classify it as a specific kind of triad;
    /// essentially, a safe way to cast
    pub fn cast<K: TriadKind>(self) -> Triad<K> {
        if let Some(_) = Triads::iter().filter(|i| i.is::<K>()).next() {
            return Triad {
                notes: self.notes,
                _class: core::marker::PhantomData::<K>,
            };
        }
        panic!("Impossible! The given triad does not match the specified kind...");
    }
}

impl Triad<Augmented> {
    /// Create a new [augmented](Augmented) triad.
    pub fn augmented(root: Note) -> Self {
        Self::from_root(root)
    }
}

impl Triad<Diminished> {
    /// Create a new [diminished](Diminished) triad.
    pub fn diminished(root: Note) -> Self {
        Self::from_root(root)
    }
}

impl Triad<Major> {
    /// Create a new [major](Major) triad.
    pub fn major(root: Note) -> Self {
        Self::from_root(root)
    }
}

impl Triad<Minor> {
    /// Create a new [minor](Minor) triad.
    pub fn minor(root: Note) -> Self {
        Self::from_root(root)
    }
}
