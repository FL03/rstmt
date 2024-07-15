/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use num::traits::{FromPrimitive, Num, Signed};

pub fn amod<T>(value: T, m: T) -> T
where
    T: Copy + Num + PartialOrd + Signed,
{
    let val = value % m;
    if val >= T::zero() {
        return val;
    }
    val + m
}

pub fn pymod<T>(lhs: T, rhs: T) -> T where T: Copy + Num + PartialOrd {
    let r = lhs % rhs;
    if (r < T::zero() && rhs > T::zero()) || (r > T::zero() && rhs < T::zero()) {
        r + rhs
    } else {
        r
    }
}

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
/// A utilitarian function computing the absolute value of the remainder between
/// the gvien value and the number of notes in the chromatic scale.
pub fn pmodulo<T>(value: T) -> T
where
    T: Copy + FromPrimitive + Num + PartialOrd + Signed,
{
    absmod(value, T::from_i8(crate::MODULUS).unwrap())
}
