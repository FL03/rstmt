/*
    Appellation: precision <module>
    Created At: 2026.01.10:09:29:19
    Contrib: @FL03
*/

/// The [`TruncDiv`] trait defines an operator for truncated division, which discards the
/// fractional part of the quotient, returning only the integer component. This differs from
/// `div_floor` in that it truncates towards zero rather than towards negative infinity.
pub trait TruncDiv<Rhs = Self> {
    type Output;

    fn div_trunc(self, rhs: Rhs) -> Self::Output;
}

/*
 ************* Implementations *************
*/

use num_traits::Float;

impl<T> TruncDiv for T
where
    T: Float,
{
    type Output = T;

    fn div_trunc(self, rhs: T) -> Self::Output {
        (self / rhs).trunc()
    }
}
