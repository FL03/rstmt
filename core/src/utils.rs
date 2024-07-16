/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use num::traits::{Num, Signed};

/// Computes the `absmod` of the value given a modulus.
/// The modulo is calculated before determining if
/// the value is positive or negative.
/// If negative, the value is reflected by adding the modulus
/// before taking the modulo again.
///
/// # Example
///
/// ```rust
/// use rstmt_core::absmod;
///
/// assert_eq!(absmod(22, 12), 22 % 12);
/// assert_ne!(absmod(-17, 12), (-17 % 12).abs());
/// assert_eq!(absmod(-17, 12), 7);
/// ```
pub fn absmod<T>(value: T, m: T) -> T
where
    T: Copy + Num + PartialOrd + Signed,
{
    let val = value % m;
    if val >= T::zero() {
        return val;
    }
    ((val + m) % m).abs()
}
/// A functional implementation of python's `%` operator. The implementation
/// is unique in its handling of negative values, uniquely preseving the sign
/// of the _denominator_.
pub fn pymod<T>(lhs: T, rhs: T) -> T
where
    T: Copy + Num + PartialOrd,
{
    let r = lhs % rhs;
    if (r < T::zero() && rhs > T::zero()) || (r > T::zero() && rhs < T::zero()) {
        r + rhs
    } else {
        r
    }
}
