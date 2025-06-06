/*
    Appellation: lpr <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// # LPR Transformations
///
/// The neo-Riemannian theory focuses on three primary transformations, namely: leading (L),
/// parallel (P), and relative (R). Each transformation operates on a particular chord factor
/// determined by the class of the triad. Additionally, each transformation is its own inverse
/// and they may be chained together to form a sequence of transformations.
///
/// ### Leading (L)
///
/// ### Parallel (P)
///
/// Parallel transformations work by making semitonal adjustments to the [`third`](crate::Factors::Third)
/// factor of the triad, leaving the root and fifth factors unchanged. Applying a parallel
/// transformation to a major triad results in a minor triad and vice versa.
///
/// #### _Example_
///
/// Apply a single parallel C-Major triad applying a single parallel transformation returns a c-minor triad
///
/// `CM(0, 4, 7) -P-> Cm(0, 3, 7)`
///
///```rust
/// use rstmt_core::Note;
/// use rstmt_neo::Triad;
///
/// let c_major = Triad::major(Note::from_pitch(0)); // C-Major(0, 4, 7)
/// let c_minor = Triad::minor(Note::from_pitch(0)); // C-Minor(0, 3, 7)
/// assert_eq!(c_major.parallel(), Ok(c_minor));     // C-Major(0, 4, 7) -> C-Minor(0, 3, 7)
/// assert_eq!(c_minor.parallel(), Ok(c_major));     // C-Minor(0, 3, 7) -> C-Major(0, 4, 7)  
/// ```
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
}
