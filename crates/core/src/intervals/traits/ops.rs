/*
    Appellation: ops <module>
    Created At: 2025.12.31:18:11:55
    Contrib: @FL03
*/
use crate::intervals::StepSize;
use num_traits::NumOps;

/// [`StepSizeOps`] provides a generic trait for types that can be operated on with
/// [`StepSize`] values.
pub trait StepSizeOps
where
    Self: Sized + NumOps<StepSize, Self> + NumOps<Self, StepSize>,
{
}

impl<T> StepSizeOps for T where T: Sized + NumOps<StepSize, T> + NumOps<T, StepSize> {}
