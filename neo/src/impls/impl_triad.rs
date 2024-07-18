/*
    Appellation: impl_triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

use rstmt::Note;

use crate::triad::{Augmented, Diminished, Major, Minor, Triad, TriadKind};

impl Triad<Augmented> {
    pub fn augmented(root: Note) -> Self {
        unimplemented!("Augmented triads are not yet implemented.")
    }
}
