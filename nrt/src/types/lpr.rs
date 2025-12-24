/*
    Appellation: transform <module>
    Contrib: @FL03
*/
use crate::error::TriadError;
use crate::traits::{RawTriad, TriadCls};
use crate::triad::TriadBase;
use num_traits::{FromPrimitive, Num, ToPrimitive};
use rstmt::{PitchMod, TryApply};

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
    derive(serde::Deserialize, serde::Serialize),
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
    /// a functional constructor returning a [`Leading`](LPR::Leading) transformation
    pub const fn leading() -> Self {
        LPR::Leading
    }
    /// a functional constructor returning a [`Parallel`](LPR::Parallel) transformation
    pub const fn parallel() -> Self {
        LPR::Parallel
    }
    /// a functional constructor returning a [`Relative`](LPR::Relative) transformation
    pub const fn relative() -> Self {
        LPR::Relative
    }
    pub fn iter() -> LPRIter {
        use strum::IntoEnumIterator;
        <LPR as IntoEnumIterator>::iter()
    }
    /// applies the current transformation onto the given triad, returning a new triad
    pub fn apply<S, T, K, K2>(&self, triad: &TriadBase<S, K, T>) -> TriadBase<S, K::Rel, T>
    where
        S: RawTriad<Elem = T>,
        K: TriadCls<Rel = K2>,
        K2: TriadCls<Rel = K>,
        T: Copy + FromPrimitive + ToPrimitive + Num + PitchMod<Output = T>,
    {
        self.try_apply(triad)
            .expect("Failed to apply the transformation onto the triad.")
    }
    /// Apply a transformation to a triad
    pub fn try_apply<S, T, K, K2>(
        &self,
        triad: &TriadBase<S, K, T>,
    ) -> Result<TriadBase<S, K::Rel, T>, TriadError>
    where
        S: RawTriad<Elem = T>,
        K: TriadCls<Rel = K2>,
        K2: TriadCls<Rel = K>,
        T: Copy + FromPrimitive + ToPrimitive + Num + PitchMod<Output = T>,
    {
        let x = *triad.chord().root();
        let y = *triad.chord().third();
        let z = *triad.chord().fifth();

        let notes: [T; 3];
        if triad.is_major() {
            match self {
                LPR::Leading => {
                    notes = [y, z, (x - T::one()).pmod()];
                }
                LPR::Parallel => {
                    notes = [x, (y - T::one()).pmod(), z];
                }
                LPR::Relative => {
                    notes = [(z + T::from_u8(2).unwrap()).pmod(), x, y];
                }
            }
        } else if triad.is_minor() {
            match self {
                LPR::Leading => {
                    notes = [(z + T::one()).pmod(), x, y];
                }
                LPR::Parallel => {
                    notes = [x, (y + T::one()).pmod(), z];
                }
                LPR::Relative => {
                    notes = [y, z, (x - T::from_u8(2).unwrap()).pmod()];
                }
            }
        } else {
            return Err(TriadError::InvalidTriadClass);
        }

        Ok(TriadBase {
            chord: S::from_arr(notes),
            class: triad.class().rel(),
            octave: triad.octave,
        })
    }
}

impl<S, T, K, Q> TryApply<&TriadBase<S, K, T>> for LPR
where
    K: TriadCls<Rel = Q>,
    Q: TriadCls<Rel = K>,
    S: RawTriad<Elem = T>,
    T: Copy + FromPrimitive + ToPrimitive + Num + PitchMod<Output = T>,
{
    type Output = TriadBase<S, K::Rel, T>;
    type Error = TriadError;

    fn try_apply(&self, rhs: &TriadBase<S, K, T>) -> Result<Self::Output, Self::Error> {
        LPR::try_apply(self, rhs)
    }
}

impl TryFrom<char> for LPR {
    type Error = TriadError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        use LPR::*;
        match value.to_ascii_lowercase() {
            'l' => Ok(Leading),
            'p' => Ok(Parallel),
            'r' => Ok(Relative),
            v => Err(TriadError::ParseTransformationCharError(v)),
        }
    }
}

macro_rules! impl_from_lpr {
    ($($T:ty),* $(,)?) => {
        $(impl_from_lpr! { @impl $T })*
    };
    (@impl $T:ty) => {
        impl From<LPR> for $T {
            fn from(value: LPR) -> Self {
                match value {
                    LPR::Leading => 0,
                    LPR::Parallel => 1,
                    LPR::Relative => 2,
                }
            }
        }

        impl From<$T> for LPR {
            fn from(value: $T) -> Self {
                match value % 3 {
                    0 => LPR::Leading,
                    1 => LPR::Parallel,
                    2 => LPR::Relative,
                    _ => unreachable!("Modular arithmetic failed"),
                }
            }
        }
    };
}

impl_from_lpr! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }
