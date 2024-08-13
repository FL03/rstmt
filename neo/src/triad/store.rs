/*
    Appellation: store <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::transform::LPR;
use crate::triad::Triads;
use crate::{ChordFactor, Factors, TriadError};
use itertools::Itertools;
use rstmt::{Fifth, IntervalOps, Note, Third};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct BaseTriad {
    pub root: Note,
    pub third: Note,
    pub fifth: Note,
}

impl BaseTriad {
    pub fn new(root: Note, third: Note, fifth: Note) -> Self {
        Self { root, third, fifth }
    }

    pub fn augmented(root: Note) -> Self {
        Self::new(root, root.add_major_third(), root.add_augmented_fifth())
    }

    pub fn diminished(root: Note) -> Self {
        Self::new(root, root.add_minor_third(), root.add_diminished_fifth())
    }

    pub fn major(root: Note) -> Self {
        Self {
            root,
            third: root.add_major_third(),
            fifth: root.add_perfect_fifth(),
        }
    }

    pub fn minor(root: Note) -> Self {
        Self {
            root,
            third: root.add_minor_third(),
            fifth: root.add_perfect_fifth(),
        }
    }

    pub fn from_array([root, third, fifth]: [Note; 3]) -> Self {
        Self { root, third, fifth }
    }

    pub fn from_tuple((root, third, fifth): (Note, Note, Note)) -> Self {
        Self { root, third, fifth }
    }
    pub fn try_from_arr(notes: [Note; 3]) -> Result<Self, TriadError> {
        for (i, j, k) in notes.into_iter().circular_tuple_windows() {
            let triad = Self::new(i, j, k);
            if triad.is_valid() {
                return Ok(triad);
            }
        }
        Err(TriadError::invalid_triad("The triad is invalid."))
    }
    /// Constructs a tuple of the triad's notes.
    pub fn as_tuple(&self) -> (Note, Note, Note) {
        (self.root, self.third, self.fifth)
    }
    /// Constructs a mutable tuple of the triad's notes.
    pub fn as_mut_tuple(&mut self) -> (&mut Note, &mut Note, &mut Note) {
        (&mut self.root, &mut self.third, &mut self.fifth)
    }
    /// Constructs an array of the triad's notes.
    pub fn as_array(&self) -> [Note; 3] {
        [self.root, self.third, self.fifth]
    }
    /// Constructs a mutable array of the triad's notes.
    pub fn as_mut_array(&mut self) -> [&mut Note; 3] {
        [&mut self.root, &mut self.third, &mut self.fifth]
    }
    /// Constructs a [Vec] of the triad's notes.
    pub fn as_vec(&self) -> Vec<Note> {
        vec![self.root, self.third, self.fifth]
    }
    /// Checks if the triad is valid; computes the intervals between the notes
    /// interpreting any errors as invalid configurations.
    pub fn is_valid(&self) -> bool {
        self.root_to_third().is_ok()
            && self.third_to_fifth().is_ok()
            && self.root_to_fifth().is_ok()
    }
    /// Classifies the triad as a [Triads] variant.
    pub fn classify(&self) -> Result<Triads, TriadError> {
        let rt = self.root_to_third()?;
        let rf = self.root_to_fifth()?;

        let class = match rf {
            Fifth::Perfect => match rt {
                Third::Major => Triads::Major,
                Third::Minor => Triads::Minor,
            },
            Fifth::Augmented => Triads::Augmented,
            Fifth::Diminished => Triads::Diminished,
        };
        Ok(class)
    }
    /// Returns an owned reference to the root (first) note of the triad.
    pub fn root(&self) -> Note {
        self.root
    }
    /// Returns a mutable reference to the root (first) note of the triad.
    pub fn root_mut(&mut self) -> &mut Note {
        &mut self.root
    }
    /// Returns an owned reference to the middle (seoncd) note of the triad.
    pub const fn third(&self) -> Note {
        self.third
    }
    /// Returns a mutable reference to the middle (second) note of the triad.
    pub fn third_mut(&mut self) -> &mut Note {
        &mut self.third
    }
    /// Returns an owned reference to the final note of the triad.
    pub fn fifth(&self) -> Note {
        self.fifth
    }
    /// Returns a mutable reference to the final note of the triad.
    pub fn fifth_mut(&mut self) -> &mut Note {
        &mut self.fifth
    }
    /// Returns the distance, or [interval](rstmt::Interval), between the [root](Factors::Root)
    /// and [third](Factors::Third) factors. If the measured interval is not a third, the
    /// function will throw an error.
    pub fn root_to_third(&self) -> Result<Third, TriadError> {
        Third::new(self.root, self.third).map_err(|_e| {
            TriadError::invalid_interval("The interval between the root and third is not a third.")
        })
    }
    /// Returns the distance, or [interval](rstmt::Interval), between the [root](Factors::Root)
    /// and [fifth](Factors::Fifth) factors. If the measured interval is not a fifth, the
    /// function will throw an error.
    pub fn root_to_fifth(&self) -> Result<Fifth, TriadError> {
        Fifth::new(self.root, self.fifth).map_err(|_e| {
            TriadError::invalid_interval("The interval between the root and fifth is not a fifth.")
        })
    }
    /// Returns the distance, or [interval](rstmt::Interval), between the [third](Factors::Third)
    /// and [fifth](Factors::Fifth) factors. If the measured interval is not a fifth, the
    /// function will throw an error.
    pub fn third_to_fifth(&self) -> Result<Third, TriadError> {
        Third::new(self.third, self.fifth).map_err(|_e| {
            TriadError::invalid_interval("The interval between the third and fifth is not a fifth.")
        })
    }
    /// Applies the given transformation to the chord, returning a new [BaseTriad] instance.
    pub fn transform(mut self, lpr: LPR) -> BaseTriad {
        lpr.transform(&mut self)
    }
    /// Updates the note specified and provided by the [ChordFactor] enum.
    pub fn update(&mut self, factor: ChordFactor<Note>) {
        match factor {
            ChordFactor::Root(data) => self.root = data,
            ChordFactor::Third(data) => self.third = data,
            ChordFactor::Fifth(data) => self.fifth = data,
        }
    }
}

impl core::fmt::Display for BaseTriad {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}, {}, {}", self.root, self.third, self.fifth)
    }
}

impl core::ops::Index<Factors> for BaseTriad {
    type Output = Note;

    fn index(&self, index: Factors) -> &Self::Output {
        match index {
            Factors::Root => &self.root,
            Factors::Third => &self.third,
            Factors::Fifth => &self.fifth,
        }
    }
}

impl core::ops::IndexMut<Factors> for BaseTriad {
    fn index_mut(&mut self, index: Factors) -> &mut Self::Output {
        match index {
            Factors::Root => &mut self.root,
            Factors::Third => &mut self.third,
            Factors::Fifth => &mut self.fifth,
        }
    }
}
