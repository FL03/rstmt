/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Major, TriadCls, TriadKind, Triads};
use crate::transform::{Transform, LPR};
use crate::TriadError;
use core::marker::PhantomData;
use itertools::Itertools;
use rstmt::{Fifth, Note, Third};

fn _constructor<K>(data: &[Note; 3]) -> Result<Triad<K>, TriadError>
where
    K: TriadKind,
{
    for (&a, &b, &c) in data.iter().circular_tuple_windows() {
        if let Ok(_cls) = Triads::try_from_notes(a, b, c) {
            return Ok(Triad::new(a));
        }
    }
    Err(TriadError::InvalidInterval(
        "Failed to find the required relationships within the given notes...".into(),
    ))
}

fn _transform<K>(triad: Triad<K>, lpr: LPR) -> Result<Triad<Triads>, TriadError>
where
    K: TriadKind,
{
    use rstmt::Intervals::{Semitone, Tone};
    let (mut r, mut t, mut f) = triad.into_tuple();
    match <K>::root_to_third() {
        Third::Major => match lpr {
            LPR::L => r -= Semitone,
            LPR::P => t -= Semitone,
            LPR::R => f += Tone,
        },
        Third::Minor => match lpr {
            LPR::L => r += Semitone,
            LPR::P => t += Semitone,
            LPR::R => f -= Tone,
        },
    };
    // _constructor::<J>([r, t, f])
    Triads::try_from_notes(r, t, f)
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

impl<K: TriadCls> Triad<K> {
    pub const LEN: usize = 3;
    /// Returns a new instance of [Triad] from a root note and a classifying type;
    pub fn as_dyn(&self) -> Triad<Triads> {
        Triad {
            notes: self.notes,
            _class: PhantomData::<Triads>,
        }
    }
    /// Returns the notes as an array
    pub fn as_array(&self) -> &[Note; 3] {
        &self.notes
    }

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

    /// Swaps the classifying type of the triad;
    /// useful for converting from dynamically typed triads to statically typed triads.
    pub fn swap_kind<J: TriadKind>(&self) -> Triad<J> {
        Triad {
            notes: self.notes,
            _class: PhantomData::<J>,
        }
    }

    /// Returns the root note of the triad
    pub fn root(&self) -> Note {
        self.notes[0]
    }
    /// Returns a mutable reference to the root note of the triad
    pub fn root_mut(&mut self) -> &mut Note {
        &mut self.notes[0]
    }
    /// Returns the distance (interval) between the root and the third
    pub fn root_to_third(&self) -> Result<Third, TriadError> {
        Third::interval(self.notes[0], self.notes[1]).map_err(|_| {
            TriadError::InvalidInterval(
                "Failed to compute the interval between the root and the third...".into(),
            )
        })
    }
    /// Returns the distance (interval) between the root and the fifth
    pub fn root_to_fifth(&self) -> Result<Fifth, TriadError> {
        Fifth::interval(self.notes[0], self.notes[2]).map_err(|_| {
            TriadError::InvalidInterval(
                "Failed to compute the interval between the root and the fifth...".into(),
            )
        })
    }

    /// Returns the third note of the triad
    pub fn third(&self) -> Note {
        self.notes[1]
    }
    /// Returns a mutable reference to the third note of the triad
    pub fn third_mut(&mut self) -> &mut Note {
        &mut self.notes[1]
    }
    /// Returns the distance (interval) between the third and the fifth
    pub fn third_to_fifth(&self) -> Result<Third, TriadError> {
        Third::interval(self.notes[1], self.notes[2]).map_err(|_| {
            TriadError::InvalidInterval(
                "Failed to compute the interval between the third and the fifth...".into(),
            )
        })
    }
    /// Returns the final note of the triad
    pub fn fifth(&self) -> Note {
        self.notes[2]
    }
    /// Returns a mutable reference to the final note of the triad
    pub fn fifth_mut(&mut self) -> &mut Note {
        &mut self.notes[2]
    }
    /// Returns a new instance of [Triad] with the notes in reverse order
    pub fn reversed(&self) -> Self {
        Self {
            notes: [self.notes[2], self.notes[1], self.notes[0]],
            _class: PhantomData::<K>,
        }
    }
}
impl<K: TriadKind> Triad<K> {
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
    /// Create a new triad from a slice of notes;
    /// unlike [`try_from_notes`](Triad::try_from_notes), this function
    /// iterates through the given notes to discover __**any**__ configuration
    /// that is valid. If no valid configuration is found, an error is returned.
    pub fn try_from_arr(notes: [Note; 3]) -> Result<Self, TriadError> {
        for (&a, &b, &c) in notes.iter().circular_tuple_windows() {
            if let Ok(_) = Triads::try_from_notes(a, b, c) {
                return Ok(Triad::new(a));
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
        let a = Third::try_from(*(third - root).pitch())?;
        // compute the interval between the third and the fifth
        let b = Third::try_from(*(fifth - third).pitch())?;
        if a == K::root_to_third() && b == K::third_to_fifth() {
            return Ok(Self {
                _class: PhantomData::<K>,
                notes: [root, third, fifth],
            });
        }
        Err(TriadError::invalid_triad(
            "The given notes do not form a valid triad...",
        ))
    }
    /// Returns the [class](Triads) of the triad
    pub fn class(&self) -> Triads {
        K::class()
    }
    ///
    pub fn is_valid(&self) -> bool {
        let (r, t, f) = (self.notes[0], self.notes[1], self.notes[2]);
        let a = Third::try_from(*(t - r).pitch()).unwrap();
        let b = Third::try_from(*(f - t).pitch()).unwrap();
        a == K::root_to_third() && b == K::third_to_fifth()
    }
    /// Returns an iterator over the notes of the triad
    pub fn iter(&self) -> core::slice::Iter<Note> {
        self.notes.iter()
    }
    /// Returns a mutable iterator over the notes of the triad
    pub fn iter_mut(&mut self) -> core::slice::IterMut<Note> {
        self.notes.iter_mut()
    }

    /// Transforms the triad using one of the [LPR](crate::transform::LPR) transformations.
    /// Each transformation
    pub fn transform(self, lpr: LPR) -> Triad<Triads> {
        _transform(self, lpr).unwrap()
    }
}

impl<K> Transform<LPR> for Triad<K>
where
    K: TriadKind,
{
    type Output = Triad<Triads>;

    fn transform(self, lpr: LPR) -> Self::Output {
        self.transform(lpr)
    }
}
