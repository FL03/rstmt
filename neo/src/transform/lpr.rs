/*
    Appellation: lpr <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::Triad;

///
///
/// ### Leading (L)
///
/// ### Parallel (P)
///
/// Parallel transformations affect the third by half a step while leaving the root and fifth
/// unchanged. When applied on a major triad, the transformation results in a minor and vise
/// versa. For example, applying a single parallel transformation to an F major triad gives us
/// an F minor triad.
///
/// ### Relative (R)
///
/// A relative transformation transforms a triad into its relative. The transformation is more
/// involved than either leading or parallel transformations as they depend upon
#[derive(
    Clone,
    Copy,
    Debug,
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
    strum::VariantArray,
    strum::VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "UPPERCASE")
)]
#[repr(u8)]
#[strum(serialize_all = "UPPERCASE")]
pub enum LPR {
    #[cfg_attr(feature = "serde", serde(alias = "l", alias = "leading"))]
    L = 0,
    #[cfg_attr(feature = "serde", serde(alias = "p", alias = "parallel"))]
    P = 1,
    #[cfg_attr(feature = "serde", serde(alias = "r", alias = "relative"))]
    R = 2,
}

impl LPR {
    /// A functional constructor for the `leading` transformation
    pub fn leading() -> Self {
        LPR::L
    }
    /// A functional constructor for the `parallel` transformation
    pub fn parallel() -> Self {
        LPR::P
    }
    /// A functional constructor for the `relative` transformation
    pub fn relative() -> Self {
        LPR::R
    }
    /// Apply the current transformation to the given triad;
    /// returns a [Triad] with the new notes and classification
    pub fn apply<K>(self, triad: &mut Triad<K>) -> Triad<crate::triad::Triads> {
        use rstmt::{
            Interval::{Semitone, Tone},
            Third::*,
        };
        let rt = triad
            .root_to_third()
            .expect("The given triad contained an invalid interval between the root and third!");
        let [r, t, f] = triad.as_mut_array();
        match rt {
            Major => match self {
                LPR::L => *r -= Semitone,
                LPR::P => *t -= Semitone,
                LPR::R => *f += Tone,
            },
            Minor => match self {
                LPR::L => *f += Semitone,
                LPR::P => *t += Semitone,
                LPR::R => *r -= Tone,
            },
        };

        Triad::try_from_arr(dbg!([*r, *t, *f])).expect(
            "Transformation Error: the triad could not be constructed from the given notes.",
        )
    }

    pub fn dyna(&self) -> Box<dyn A> {
        match self {
            LPR::L => Box::new(0),
            LPR::P => Box::new(0),
            LPR::R => Box::new(0),
        }
    }
}

pub trait A {
    fn a() -> bool
    where
        Self: Sized,
    {
        false
    }
}

impl<T> A for T {}
