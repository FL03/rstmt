/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Major, TriadKind, Triads};
use crate::transform::LPR;
use crate::{Factors, TriadError};
use core::marker::PhantomData;
use itertools::Itertools;
use rstmt::{Note, Third};

fn _constructor(data: &[Note; 3]) -> Result<Triad, TriadError> {
    for (&a, &b, &c) in data.iter().circular_tuple_windows() {
        if let Ok(_) = Triads::try_from_notes(a, b, c) {
            return Ok(Triad::new(a));
        }
    }
    Err(TriadError::InvalidInterval(
        "Failed to find the required relationships within the given notes...".into(),
    ))
}

fn _transform(triad: &Triad, lpr: LPR) -> Result<Triad, TriadError> {
    use rstmt::{
        Intervals::{Semitone, Tone},
        Third,
    };
    use Factors::*;

    let (rt, _tf, _rf) = triad.class().intervals();
    // match rt {
    //     Third::Major => match lpr {
    //         LPR::L => triad[Root] -= Semitone,
    //         LPR::P => triad[Third] -= Semitone,
    //         LPR::R => triad[Fifth] += Tone,
    //     },
    //     Third::Minor => match lpr {
    //         LPR::L => triad[Fifth] += Semitone,
    //         LPR::P => triad[Third] += Semitone,
    //         LPR::R => triad[Root] -= Tone,
    //     },
    // };

    unimplemented!();
}

/// A triad speaks to any chord composed of three notes, each of which maintain
/// particular relationships to one another. The most common triad is the major
/// triad, which is composed of a root note, a major third, and a perfect fifth.
/// The minor triad is composed of a root note, a minor third, and a perfect fifth.
///
/// Triads can be created either by specifying the root note and providing the
/// classifying type, or by providing an array of notes. When providing an array
/// of notes, the system works to ensure the final configuration of notes is valid.
/// This is done by iterating over the notes and checking if the intervals satisfy
/// the requirements of the given triad class.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]

pub struct Triad<K = Major> {
    pub(crate) notes: [Note; 3],
    pub(crate) _class: PhantomData<K>,
}

impl<K> Triad<K>
where
    K: TriadKind,
{
    pub const LEN: usize = 3;
    /// Create a new triad from a root note.
    pub fn new(root: Note) -> Self {
        let (rt, tf) = K::thirds();
        let octave = *root.octave();
        let t = Note::new(octave, root.pitch() + rt.value());
        let f = Note::new(octave, t.pitch() + tf.value());
        Self {
            _class: PhantomData::<K>,
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
                _class: PhantomData::<K>,
                notes,
            })
        } else {
            Err(TriadError::invalid_triad(
                "The given notes do not form a valid triad...",
            ))
        }
    }
    /// Returns the notes as an array
    pub fn as_array(&self) -> &[Note; 3] {
        &self.notes
    }
    /// Returns a slice containing the notes of the triad
    pub fn as_slice(&self) -> &[Note] {
        &self.notes
    }
    /// Returns the notes as a three-tuple
    pub fn as_tuple(&self) -> (Note, Note, Note) {
        (self.notes[0], self.notes[1], self.notes[2])
    }
    /// Consumes and returns an array of notes
    pub fn into_array(self) -> [Note; 3] {
        self.notes
    }
    /// Consumes and returns a tuple of notes
    pub fn into_tuple(self) -> (Note, Note, Note) {
        (self.notes[0], self.notes[1], self.notes[2])
    }
    /// Returns the [class](Triads) of the triad
    pub fn class(&self) -> Triads {
        K::class()
    }
    /// Returns the root note of the triad
    pub fn root(&self) -> Note {
        self.notes[0]
    }
    /// Returns the third note of the triad
    pub fn third(&self) -> Note {
        self.notes[1]
    }
    /// Returns the final note of the triad
    pub fn fifth(&self) -> Note {
        self.notes[2]
    }
    /// Returns a new instance of [Triad] with the notes in reverse order
    pub fn reversed(&self) -> Self {
        Self {
            notes: [self.notes[2], self.notes[1], self.notes[0]],
            _class: PhantomData::<K>,
        }
    }
    /// Returns the distance (interval) between the root and the third
    pub fn root_to_third(&self) -> Third {
        K::root_to_third()
    }
    /// Returns the distance (interval) between the third and the fifth
    pub fn third_to_fifth(&self) -> Third {
        K::third_to_fifth()
    }
    /// Returns an iterator over the notes of the triad
    pub fn iter(&self) -> core::slice::Iter<Note> {
        self.notes.iter()
    }
    /// Returns a mutable iterator over the notes of the triad
    pub fn iter_mut(&mut self) -> core::slice::IterMut<Note> {
        self.notes.iter_mut()
    }
    // #
    pub fn transform(&self, lpr: LPR) -> Self {
        unimplemented!();
    }
}

impl<K> core::fmt::Display for Triad<K>
where
    K: TriadKind,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let (root, third, fifth) = self.as_tuple();
        write!(f, "({}, {}, {})", root, third, fifth)
    }
}
