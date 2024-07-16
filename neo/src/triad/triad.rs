/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Major, TriadKind};
use crate::{Factors, TriadError};
use core::marker::PhantomData;
use rstmt::{Note, Third};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]

pub struct Triad<K = Major> {
    pub(crate) kind: PhantomData<K>,
    pub notes: [Note; 3],
}

impl<K> Triad<K>
where
    K: TriadKind,
{
    pub fn new(root: Note) -> Self {
        let (rt, tf) = K::thirds();
        let octave = *root.octave();
        let t = Note::from_pitch(root.pitch() + rt.value()).with_octave(octave);
        let f = Note::from_pitch(t.pitch() + tf.value()).with_octave(octave);
        Self {
            kind: PhantomData::<K>,
            notes: [root, t, f],
        }
    }
    /// Create a new triad from a slice of notes.
    pub fn from_slice(notes: [Note; 3]) -> Result<Self, TriadError> {
        let (r, t, f) = (notes[0], notes[1], notes[2]);
        // compute the interval between the root and the third
        let a = Third::try_from(*(t - r).pitch())?;
        // compute the interval between the third and the fifth
        let b = Third::try_from(*(f - t).pitch())?;
        if a == K::root_to_third() && b == K::third_to_fifth() {
            Ok(Self {
                kind: PhantomData::<K>,
                notes,
            })
        } else {
            Err(TriadError::invalid_triad(r, t, f))
        }
    }

    pub fn as_slice(&self) -> &[Note; 3] {
        &self.notes
    }

    pub fn as_tuple(&self) -> (Note, Note, Note) {
        (self.notes[0], self.notes[1], self.notes[2])
    }

    pub fn into_tuple(self) -> (Note, Note, Note) {
        (self.notes[0], self.notes[1], self.notes[2])
    }

    pub fn into_slice(self) -> [Note; 3] {
        self.notes
    }

    pub fn root(&self) -> Note {
        self.notes[0]
    }

    pub fn third(&self) -> Note {
        self.notes[1]
    }

    pub fn fifth(&self) -> Note {
        self.notes[2]
    }

    pub fn reversed(&self) -> Self {
        Self {
            kind: PhantomData::<K>,
            notes: [self.notes[2], self.notes[1], self.notes[0]],
        }
    }

    pub fn root_to_third(&self) -> Third {
        K::root_to_third()
    }

    pub fn third_to_fifth(&self) -> Third {
        K::third_to_fifth()
    }
}

impl<K> core::fmt::Display for Triad<K>
where
    K: TriadKind,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(&self.notes[0].to_string())?;
        f.write_str(&self.notes[1].to_string())?;
        f.write_str(&self.notes[2].to_string())
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
