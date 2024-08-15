/*
    Appellation: impl_triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use rstmt::Note;
use strum::IntoEnumIterator;

use crate::triad::kinds::*;
use crate::triad::{Triad, Triads};

impl Triad<dyn Kind<Class = Triads>> {
    /// Consumes the instance to classify it as a specific kind of triad;
    /// essentially, a safe way to cast "dynamic" instances to "static" ones.
    pub fn cast<K>(self) -> Triad<K>
    where
        K: Kind<Class = Triads> + 'static,
    {
        if let Some(_) = Triads::iter().find(|i| i.is::<K>()) {
            return Triad {
                notes: self.notes,
                _class: core::marker::PhantomData::<K>,
            };
        }
        panic!("Impossible! The given triad does not match the specified kind...");
    }
}

impl Triad<Triads> {
    /// Consumes the instance to classify it as a specific kind of triad;
    /// essentially, a safe way to cast "dynamic" instances to "static" ones.
    pub fn cast<K>(self) -> Triad<K>
    where
        K: Kind<Class = Triads> + 'static,
    {
        if let Some(_) = Triads::iter().find(|i| i.is::<K>()) {
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
