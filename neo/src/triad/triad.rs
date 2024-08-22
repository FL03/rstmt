/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Kind, Major, TriadKind, Triads};
use crate::transform::LPR;
use crate::NeoError;
use core::marker::PhantomData;
use itertools::Itertools;
use rstmt::{Fifth, Note, Third};

/// # Triad
///
/// A triad is defined to be a type of chord compose of three notes that maintain specific
/// intervalic relationships with each other. The three notes are often called _chord factors_
/// and are referenced as the root, third, and fifth. The root note is the note from which the
/// chord is built, the third is the note that is a third above the root, and the fifth is the
/// note that is a fifth above the root. Therefore, it follows that the interval between the
/// third and fifth factors is also considered to be a third.
///
/// ## Classification
///
/// Considering the compositional restrictions of the object, triads are able to be classified
/// as either major, minor, augmented, or diminished. The classification of a triad is
/// determined by the intervals between the root and the third, the third and the fifth, and
/// the root and the fifth factors.
///
///
/// ## Creation Routines
///
/// Triads can be created either by specifying the root note and providing the
/// classifying type, or by providing an array of notes. When providing an array
/// of notes, the system works to ensure the final configuration of notes is valid.
/// This is done by iterating over the notes and checking if the intervals satisfy
/// the requirements of the given triad class.
///
/// ### _Example_
///
/// For example, a C-Major triad is composed of the notes C, E, and G.
/// The interval between C and E is a major third, while the interval between
/// E and G is a minor third, leaving the final interval between C and G as a
/// perfect fifth.
///
/// ```rust
/// use rstmt_core::Note;
/// use rstmt_neo::Triad;
///
/// let c_major = dbg!(Triad::major(Note::from_pitch(0)));
/// assert_eq!(c_major.root(), Note::from_pitch(0));  // C
/// assert_eq!(c_major.third(), Note::from_pitch(4)); // E
/// assert_eq!(c_major.fifth(), Note::from_pitch(7)); // G
/// ```
///
/// ## Side Note
///
/// From a topological perspective, triads represent a 2-simplex where the three notes are
/// considered to be the vertices while the intervals between the notes are the edges.
///
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]

pub struct Triad<K: ?Sized = Major> {
    pub(crate) notes: [Note; 3],
    pub(crate) _class: PhantomData<K>,
}

impl<K> Triad<K> {
    pub const LEN: usize = 3;
    /// Create a new triad from a root note; the classifying type is used to statically
    /// determine the composition of the triad, leveraging the defined intervals to
    /// compute the third and fifth factors.
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
    /// Create a new triad from a slice of notes; unlike [`try_from_notes`](Triad::try_from_notes),
    /// this function iterates through the given notes to discover __**any**__ configuration
    /// that is valid; returns an error if no configuration of notes contains the required
    /// relationships.
    pub fn try_from_arr(notes: [Note; 3]) -> Result<Self, NeoError> {
        for (&a, &b, &c) in notes.iter().circular_tuple_windows() {
            println!("{a} {b} {c}");
            if let Ok(triad) = Triad::try_from_notes(a, b, c) {
                return Ok(triad);
            } else {
                continue;
            }
        }
        Err(NeoError::invalid_triad(
            "Failed to find the required relationships within the given notes...",
        ))
    }
    /// Attempts to construct a new [Triad] from the given notes and classifying type; returns
    /// an error if the neccessary relationships are note detected within the given notes.
    pub fn try_from_notes(root: Note, third: Note, fifth: Note) -> Result<Self, NeoError> {
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
            Err(NeoError::invalid_triad(
                "Failed to detect the required intervals...",
            ))
        }
    }
    /// Returns an owned reference to the array of notes
    pub fn as_array(&self) -> &[Note; 3] {
        &self.notes
    }
    /// Returns a mutable reference to the notes as an array
    pub fn as_mut_array(&mut self) -> &mut [Note; 3] {
        &mut self.notes
    }
    /// Returns a new instance of [Triad] from a root note and a classifying type; dyn TriadKind<Rel = K::Rel> where K: TriadKind
    pub fn as_dyn(&self) -> Triad<dyn core::any::Any> {
        Triad {
            notes: self.notes,
            _class: PhantomData::<dyn core::any::Any>,
        }
    }
    /// Returns the classifying type of the triad
    pub fn class(&self) -> Triads
    where
        K: Kind<Class = Triads>,
    {
        <K>::class()
    }
    /// Returns the name of the class
    pub fn class_name(&self) -> &str
    where
        K: Kind,
    {
        <K>::name()
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
    ///
    pub fn edges(&self) -> Result<(Third, Third, Fifth), NeoError> {
        Ok((
            self.root_to_third()?,
            self.third_to_fifth()?,
            self.root_to_fifth()?,
        ))
    }
    /// Computes the interval between the root and the third factors
    pub fn root_to_third(&self) -> Result<Third, NeoError> {
        Third::new(self.root(), self.third())
            .map_err(|_| NeoError::invalid_interval(self.root(), self.third()))
    }
    /// Returns the distance (interval) between the root and the fifth
    pub fn root_to_fifth(&self) -> Result<Fifth, NeoError> {
        Fifth::new(self.root(), self.fifth())
            .map_err(|_| NeoError::invalid_interval(self.root(), self.fifth()))
    }
    /// Returns the distance (interval) between the third and the fifth
    pub fn third_to_fifth(&self) -> Result<Third, rstmt::Error> {
        Third::new(self.third(), self.fifth())
    }
    /// Returns a new instance of [Triad] with the notes in reverse order
    pub fn reversed(&self) -> Self {
        Self {
            notes: [self.notes[2], self.notes[1], self.notes[0]],
            _class: PhantomData::<K>,
        }
    }
    /// Cast the current triad into a new triad with a different classifying type
    pub fn cast<J>(self) -> Triad<J> {
        Triad {
            notes: self.notes,
            _class: PhantomData::<J>,
        }
    }
    /// Applies the given [LPR] transformation onto the triad.
    ///
    pub fn transform<J>(self, lpr: LPR) -> Result<Triad<J>, NeoError>
    where
        K: TriadKind<Rel = J>,
    {
        crate::transform::Transformer::new(lpr, self).apply()
    }
    /// Leading transformations make semitonal adjusments to the root of the triad;
    /// when applied to a major triad, the leading transformation decrements the root note by
    /// a semitone and while the third and fifth factors are unchanged, they are shifted down
    /// a factor becoming the root and third factors respectively allowing the root to become
    /// the fifth factor. For minor triads, the fifth factor is incremented by a semitone and
    /// the root and third factors are shifted up by a factor. The fifth factor is then moved
    /// to the root.
    ///
    pub fn leading<J>(self) -> Result<Triad<J>, NeoError>
    where
        K: TriadKind<Rel = J>,
    {
        self.transform(LPR::L)
    }
    /// Applies a parallel transformation to the triad; parallel transformations work by making
    /// semitonal adjustments to the [`third`](crate::Factors::Third) factor of the triad. If
    /// the triad is a major triad, applying a parallel transformation will result in a minor
    /// triad and vice versa.
    ///
    /// ### Example
    ///
    /// Apply a single parallel C-Major triad applying a single parallel transformation returns a c-minor triad
    ///
    /// `CM(0, 4, 7) -P-> Cm(0, 3, 7)`
    ///
    ///```rust
    /// use rstmt_core::Note;
    /// use rstmt_neo::Triad;
    ///
    /// let triad = Triad::major(Note::from_pitch(0));
    /// assert_eq!(triad.parallel(), Triad::minor(Note::from_pitch(0)));
    /// assert_eq!(triad.parallel().parallel(), triad);
    /// ```
    pub fn parallel<J>(self) -> Result<Triad<J>, NeoError>
    where
        K: TriadKind<Rel = J>,
    {
        self.transform(LPR::P)
    }
    /// Applies a relative transformation to the triad;
    pub fn relative<J>(self) -> Result<Triad<J>, NeoError>
    where
        K: TriadKind<Rel = J>,
    {
        self.transform(LPR::R)
    }
}
