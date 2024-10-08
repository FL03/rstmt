/*
    Appellation: chord <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::ChordData;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Chord<S>
where
    S: ChordData,
{
    pub(crate) len: usize,
    pub(crate) notes: S,
}

impl<S> Chord<S>
where
    S: ChordData,
{
    pub fn new() -> Self
    where
        S: Default,
    {
        Self {
            len: 0,
            notes: Default::default(),
        }
    }

    pub fn from_notes(notes: S) -> Self {
        Self {
            len: notes.len(),
            notes,
        }
    }

    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = S::Elem>,
        S: Default,
    {
        let mut notes = S::default();
        for note in iter {
            notes.push(note);
        }
        Self::from_notes(notes)
    }

    pub fn get(&self, idx: usize) -> &S::Elem {
        self.notes.get(idx)
    }

    pub fn len(&self) -> usize {
        debug_assert_eq!(
            self.len,
            self.notes.len(),
            "Chord length is inconsistent with notes length"
        );
        self.len
    }

    pub fn push(&mut self, note: S::Elem) {
        self.len += 1;
        self.notes.push(note);
    }
}
