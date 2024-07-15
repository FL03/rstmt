/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use num::traits::{FromPrimitive, Num, Signed};

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