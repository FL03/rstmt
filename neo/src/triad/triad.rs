/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Major, TriadCls, TriadKind, Triads};
use crate::transform::LPR;
use crate::TriadError;
use core::marker::PhantomData;
use itertools::Itertools;
use rstmt::{Fifth, Note, Third};

/// # Triad
///
/// A triad is a 3-note chord that is built and referenced with chord factors:
/// the root, the third, and the fifth. Each of these notes is required to
/// satisfy a particular intervalic relationship with the others. These
/// relationships are used, primarily, as the basis for classifying the triad.
/// However, knowledge of the relationships between the notes is useful
/// for building out the various creation routines and understanding the nature
/// of the triad.
///
/// For example, a C-Major triad is composed of the notes C, E, and G.
/// The interval between C and E is a major third, while the interval between
/// E and G is a minor third, leaving the final interval between C and G as a
/// perfect fifth.
///
/// ### Creation Routines
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

impl<K> Triad<K> {
    pub const LEN: usize = 3;
    /// Create a new triad from a root note.
    pub fn from_root(root: Note) -> Self
    where
        K: TriadKind,
    {
        let (rt, tf) = K::thirds();
        let octave = *root.octave();
        let t = root + rt.interval();
        let f = Note::new(octave, t.pitch() + tf.value());
        Self {
            _class: PhantomData::<K>,
            notes: [root, t, f],
        }
    }
    /// Create a new triad from a slice of notes;
    /// unlike [`try_from_notes`](Triad::try_from_notes), this function
    /// iterates through the given notes to discover __**any**__ configuration
    /// that is valid. If no valid configuration is found, an error is returned.
    pub fn try_from_arr(notes: [Note; 3]) -> Result<Self, TriadError> {
        for (&a, &b, &c) in notes.iter().circular_tuple_windows() {
            println!("{a} {b} {c}");
            if let Ok(triad) = Triad::try_from_notes(a, b, c) {
                return Ok(triad);
            } else {
                continue;
            }
        }
        Err(TriadError::InvalidInterval(
            "Failed to find the required relationships within the given notes...".into(),
        ))
    }
    /// Returns a new instance of [Triad] from a root note and a classifying type;
    /// if the given notes do not form a valid triad, an [error](TriadError) is returned.
    /// This function is useful for quickly determining whether a set of notes form a valid triad.
    pub fn try_from_notes(root: Note, third: Note, fifth: Note) -> Result<Self, TriadError> {
        // compute the interval between the root and the third
        let a = Third::new(root, third);
        // compute the interval between the third and the fifth
        let b = Third::new(third, fifth);

        if a.is_ok() && b.is_ok() {
            Ok(Self {
                _class: PhantomData::<K>,
                notes: [root, third, fifth],
            })
        } else {

            Err(TriadError::InvalidInterval(
                "Failed to compute the required intervals...".into(),
            ))
        }
    }
    /// Returns a new instance of [Triad] from a root note and a classifying type;
    pub fn as_dyn(&self) -> Triad<Triads> {
        Triad {
            notes: self.notes,
            _class: PhantomData::<Triads>,
        }
    }
    /// Returns the name of the class
    pub fn class_name(&self) -> &str
    where
        K: TriadKind,
    {
        TriadCls::named(&self._class)
    }
    /// Swaps the classifying type of the triad;
    /// useful for converting from dynamically typed triads to statically typed triads.
    pub fn swap_kind<J: TriadKind>(&self) -> Triad<J> {
        Triad {
            notes: self.notes,
            _class: PhantomData::<J>,
        }
    }
    /// Transforms the triad using one of the [LPR] transformations;
    /// see [LPR::apply] for more information.
    pub fn apply(mut self, lpr: LPR) -> Triad<Triads>
    where
        K: TriadKind,
    {
        lpr.apply(&mut self)
    }
}

impl<K> Triad<K> {
    /// Returns an owned reference to the array of notes
    pub fn as_array(&self) -> &[Note; 3] {
        &self.notes
    }
    /// Returns a mutable reference to the notes as an array
    pub fn as_mut_array(&mut self) -> &mut [Note; 3] {
        &mut self.notes
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
    /// Checks if the triad is valid; computes the intervals between the notes
    /// interpreting any errors as invalid configurations.
    pub fn is_valid(&self) -> bool {
        self.root_to_third().is_ok()
            && self.third_to_fifth().is_ok()
            && self.root_to_fifth().is_ok()
    }
    /// Returns an iterator over the notes of the triad
    pub fn iter(&self) -> core::slice::Iter<Note> {
        self.notes.iter()
    }
    /// Returns a mutable iterator over the notes of the triad
    pub fn iter_mut(&mut self) -> core::slice::IterMut<Note> {
        self.notes.iter_mut()
    }
    /// Returns the root note of the triad
    pub fn root(&self) -> Note {
        self.notes[0]
    }
    /// Returns a mutable reference to the root note of the triad
    pub fn root_mut(&mut self) -> &mut Note {
        &mut self.notes[0]
    }
    /// Returns the third note of the triad
    pub fn third(&self) -> Note {
        self.notes[1]
    }
    /// Returns a mutable reference to the third note of the triad
    pub fn third_mut(&mut self) -> &mut Note {
        &mut self.notes[1]
    }
    /// Returns the final note of the triad
    pub fn fifth(&self) -> Note {
        self.notes[2]
    }
    /// Returns a mutable reference to the final note of the triad
    pub fn fifth_mut(&mut self) -> &mut Note {
        &mut self.notes[2]
    }
    /// Returns the distance (interval) between the root and the third
    pub fn root_to_third(&self) -> Result<Third, TriadError> {
        Third::new(self.notes[0], self.notes[1]).map_err(|_| {
            TriadError::InvalidInterval(
                "Failed to compute the interval between the root and the third...".into(),
            )
        })
    }
    /// Returns the distance (interval) between the root and the fifth
    pub fn root_to_fifth(&self) -> Result<Fifth, TriadError> {
        Fifth::new(self.notes[0], self.notes[2]).map_err(|_| {
            TriadError::InvalidInterval(
                "Failed to compute the interval between the root and the fifth...".into(),
            )
        })
    }
    /// Returns the distance (interval) between the third and the fifth
    pub fn third_to_fifth(&self) -> Result<Third, TriadError> {
        Third::new(self.notes[1], self.notes[2]).map_err(|_| {
            TriadError::InvalidInterval(
                "Failed to compute the interval between the third and the fifth...".into(),
            )
        })
    }
    /// Returns a new instance of [Triad] with the notes in reverse order
    pub fn reversed(&self) -> Self {
        Self {
            notes: [self.notes[2], self.notes[1], self.notes[0]],
            _class: PhantomData::<K>,
        }
    }
}
