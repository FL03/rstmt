/*
    Appellation: impl_triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use rstmt::Note;
use strum::IntoEnumIterator;

use crate::triad::kinds::*;
use crate::triad::{Triad, Triads};

impl<K: TriadKind> Triad<K> {
    /// Checks if the triad is [augmented](Augmented).
    pub fn is_augmented(&self) -> bool {
        self.class().is_augmented()
    }
    /// Checks if the triad is [diminished](Diminished).
    pub fn is_diminished(&self) -> bool {
        self.class().is_diminished()
    }
    /// Checks if the triad is [major](Major).
    pub fn is_major(&self) -> bool {
        self.class().is_major()
    }
    /// Checks if the triad is [minor](Minor).
    pub fn is_minor(&self) -> bool {
        self.class().is_minor()
    }
}

impl Triad<Triads> {
    /// Consumes the instance to classify it as a specific kind of triad;
    /// essentially, a safe way to cast "dynamic" instances to "static" ones.
    pub fn cast<K>(self) -> Triad<K>
    where
        K: Kind + 'static,
    {
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
