/*
    Appellation: interval_base <module>
    Created At: 2025.12.21:12:52:17
    Contrib: @FL03
*/
use super::RawQuality;

#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct IntervalBase<Q, T>
where
    Q: RawQuality,
{
    pub quality: Q,
    pub distance: T,
}
