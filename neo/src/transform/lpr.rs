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
}
