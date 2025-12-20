/*
    Appellation: transform <module>
    Contrib: @FL03
*/
use crate::{Triad, TriadClass, TriadError};
use rstmt::PitchMod;

/// Enumerates primary available transformations in Neo-Riemannian theory.
///
/// Each transformation is invertible, meaning that consecutive applications of any
/// transformation will return the original triad. Furthermore, LPR transformations may be
/// chained together in discrete or continuous sequences to create complex harmonic
/// progressions.
///
/// The transformations are:
///
/// - Leading (L):
///   - [Major] given a major triad, subtract a semitone from the root and move it to the fifth
///   - [Minor] given a minor triad, add a semitone to the fifth and move it to the root
/// - Parallel (P):
///   - [Major] given a major triad, subtract a semitone from the third
///   - [Minor] given a minor triad, add a semitone to the third
/// - Relative (R):
///   - [Major] given a major triad, add a tone to the fifth and move it to the root
///   - [Minor] given a minor triad, subtract a tone from the root and move it to the fifth
///
/// These transformations can be described categorically as morphisms between various triads.
/// More specifically, they are contravariant functors between categories of triads.
///
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    variants::VariantConstructors,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::EnumIter,
    strum::EnumString,
    strum::VariantArray,
    strum::VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Deserialize, serde_derive::Serialize),
    serde(rename_all = "lowercase")
)]
#[strum(serialize_all = "lowercase")]
pub enum LPR {
    /// Leading (L) transformation
    #[default]
    #[cfg_attr(feature = "serde", serde(alias = "L", alias = "l", alias = "lead"))]
    Leading,
    /// Parallel (P) transformation
    #[cfg_attr(feature = "serde", serde(alias = "P", alias = "p", alias = "par"))]
    Parallel,
    /// Relative (R) transformation
    #[cfg_attr(feature = "serde", serde(alias = "R", alias = "r", alias = "rel"))]
    Relative,
}

impl LPR {
    pub fn iter() -> LPRIter {
        use strum::IntoEnumIterator;
        <LPR as IntoEnumIterator>::iter()
    }
    /// applies the current transformation onto the given triad, returning a new triad
    pub fn apply(&self, triad: &Triad) -> Triad {
        self.try_apply(triad).unwrap()
    }
    /// Apply a transformation to a triad
    pub fn try_apply(&self, triad: &Triad) -> Result<Triad, TriadError> {
        let [x, y, z] = triad.notes;

        let notes: [usize; 3];
        let class: TriadClass;
        match triad.class() {
            TriadClass::Major => match self {
                LPR::Leading => {
                    notes = [y, z, (x as isize - 1).pmod() as usize];
                    class = TriadClass::Minor;
                }
                LPR::Parallel => {
                    notes = [x, (y as isize - 1).pmod() as usize, z];
                    class = TriadClass::Minor;
                }
                LPR::Relative => {
                    notes = [(z + 2).pmod(), x, y];
                    class = TriadClass::Minor;
                }
            },
            TriadClass::Minor => match self {
                LPR::Leading => {
                    notes = [(z + 1).pmod(), x, y];
                    class = TriadClass::Major;
                }
                LPR::Parallel => {
                    notes = [x, (y + 1).pmod(), z];
                    class = TriadClass::Major;
                }
                LPR::Relative => {
                    notes = [y, z, (x as isize - 2).pmod() as usize];
                    class = TriadClass::Major;
                }
            },
            _ => return Err(TriadError::InvalidTriadClass),
        };

        Ok(Triad {
            notes,
            class,
            octave: triad.octave,
        })
    }
}

impl From<char> for LPR {
    fn from(value: char) -> Self {
        match value.to_ascii_lowercase() {
            'l' => LPR::Leading,
            'p' => LPR::Parallel,
            'r' => LPR::Relative,
            _ => panic!("Invalid LPR transformation; character must be 'L', 'P', or 'R'"),
        }
    }
}

impl From<usize> for LPR {
    fn from(value: usize) -> Self {
        use strum::EnumCount;
        match value % Self::COUNT {
            0 => LPR::Leading,
            1 => LPR::Parallel,
            2 => LPR::Relative,
            _ => unreachable!(),
        }
    }
}

impl From<LPR> for usize {
    fn from(value: LPR) -> Self {
        match value {
            LPR::Leading => 0,
            LPR::Parallel => 1,
            LPR::Relative => 2,
        }
    }
}
