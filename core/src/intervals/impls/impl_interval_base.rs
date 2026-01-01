/*
    Appellation: impl_interval_base <module>
    Created At: 2025.12.22:13:13:42
    Contrib: @FL03
*/
use crate::intervals::{IntervalBase, RawQuality};

impl<Q, T> IntervalBase<Q, T>
where
    Q: RawQuality,
{
    pub const fn new(quality: Q, distance: T) -> Self {
        Self {
            quality,
            steps: distance,
        }
    }
    /// returns a reference to the quality
    pub const fn quality(&self) -> &Q {
        &self.quality
    }
    /// returns a reference to the distance
    pub const fn distance(&self) -> &T {
        &self.steps
    }
}
