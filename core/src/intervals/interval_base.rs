/*
    Appellation: interval_base <module>
    Created At: 2025.12.21:12:52:17
    Contrib: @FL03
*/
use super::Quality;

pub type MajorInterval<T = f64> = IntervalBase<super::Major, T>;

#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct IntervalBase<Q, T>
where
    Q: Quality,
{
    /// the quality of the interval
    pub quality: Q,
    /// the total number of steps, or semitones, in the interval
    pub steps: T,
}
