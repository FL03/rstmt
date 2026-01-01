/*
    Appellation: lpr <module>
    Created At: 2025.12.25:17:21:05
    Contrib: @FL03
*/
use crate::error::TriadError;
use crate::traits::{Relative, TriadRepr, TriadType};
use crate::triad::TriadBase;
use num_traits::{FromPrimitive, One};
use rstmt_core::{PitchMod, TryTransform};

/// The [`LPR`] implementation enumerates the primary transformations considered within the
/// Neo-Riemannian theory. Each transformation is its own inverse (meaning consecutive
/// applications of the same transformation will return the original triad) and may be chained
/// together in both disctrete and continuous space to explore relationships between triads.
/// These transformation act on the given triad based on the quality of the interval between
/// the first and second notes (i.e., the root and third chord factors) meaning the effect of
/// the transformation is dictated by whether the triad starts with a major or minor third.
///
/// With The transformations are:
///
/// - Leading (L):
///   - [Major] subtract a semitone from the root and move it to the fifth
///   - [Minor] add a semitone to the fifth and move it to the root
/// - Parallel (P):
///   - [Major] subtract a semitone from the third
///   - [Minor] add a semitone to the third
/// - Relative (R):
///   - [Major] add a tone to the fifth and move it to the root
///   - [Minor] subtract a tone from the root and move it to the fifth
///
/// Using category theory we could define these transformations to be contravariant functors
/// mapping between _categories_ of triads.
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
    Leading = 0,
    /// Parallel (P) transformation
    #[cfg_attr(feature = "serde", serde(alias = "P", alias = "p", alias = "par"))]
    Parallel = 1,
    /// Relative (R) transformation
    #[cfg_attr(feature = "serde", serde(alias = "R", alias = "r", alias = "rel"))]
    Relative = 2,
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
    /// Apply a transformation to a triad
    pub fn transform<X, Y, E>(&self, triad: X) -> Result<Y, E>
    where
        Self: TryTransform<X, Error = E, Output = Y>,
    {
        <Self as TryTransform<X>>::try_transform(self, triad)
    }
}

impl<S, T, K> TryTransform<TriadBase<S, K, T>> for LPR
where
    K: TriadType,
    K::Rel: TriadType<Rel = K>,
    S: TriadRepr<Elem = T>,
    T: Copy
        + FromPrimitive
        + One
        + PitchMod<Output = T>
        + core::ops::Add<Output = T>
        + core::ops::Sub<Output = T>,
{
    type Output = TriadBase<S, K::Rel, T>;
    type Error = TriadError;

    fn try_transform(&self, rhs: TriadBase<S, K, T>) -> Result<Self::Output, Self::Error> {
        self.try_transform(&rhs)
    }
}

impl<S, T, K> TryTransform<&TriadBase<S, K, T>> for LPR
where
    K::Rel: TriadType<Rel = K>,
    K: TriadType,
    S: TriadRepr<Elem = T>,
    T: Copy
        + FromPrimitive
        + One
        + PitchMod<Output = T>
        + core::ops::Add<Output = T>
        + core::ops::Sub<Output = T>,
{
    type Output = TriadBase<S, K::Rel, T>;
    type Error = TriadError;

    fn try_transform(&self, rhs: &TriadBase<S, K, T>) -> Result<Self::Output, Self::Error> {
        if rhs.is_augmented() || rhs.is_diminished() {
            return Err(TriadError::InvalidTriadClass);
        }
        let &x = rhs.chord().root();
        let &y = rhs.chord().third();
        let &z = rhs.chord().fifth();

        let notes: [T; 3] = if rhs.is_major() {
            match self {
                LPR::Leading => [y, z, (x - T::one()).pmod()],
                LPR::Parallel => [x, (y - T::one()).pmod(), z],
                LPR::Relative => [(z + T::from_u8(2).unwrap()).pmod(), x, y],
            }
        } else {
            match self {
                LPR::Leading => [(z + T::one()).pmod(), x, y],
                LPR::Parallel => [x, (y + T::one()).pmod(), z],
                LPR::Relative => [y, z, (x - T::from_u8(2).unwrap()).pmod()],
            }
        };

        Ok(TriadBase {
            chord: S::from_arr(notes),
            class: <K as Relative>::rel(&rhs.class()),
            octave: rhs.octave(),
        })
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
            v => Err(TriadError::TransformationParseCharError(v)),
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
                value as $T
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
