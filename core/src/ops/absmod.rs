/*
    Appellation: absmod <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::pitch::PitchTy;

/// The [`PyMod`] trait, inpired by pythons `%` operator, describes a method for finding
/// the modulo between two numbers which, rather uniquely, preserves the sign of the
/// _denominator_.
///
/// ### Example
///
/// ```rust
/// use rstmt_core::PyMod;
///
/// let m = 12;
/// assert_eq!(22.pymod(m), 10);
/// assert_eq!(-17.pymod(m), -5);
///
/// ```
pub trait PyMod<Rhs = Self> {
    type Output;

    fn pymod(&self, rhs: Rhs) -> Self::Output;
}

/// This trait further generalizes [PyMod] by returning the absolute value of the result;
/// dropping all signs in the process. This method is particularly useful when working
/// in environments where the magnitude of the result is more important than the sign.
///
/// ### Example
pub trait AbsMod<Rhs = Self> {
    type Output;

    fn absmod(&self, rhs: Rhs) -> Self::Output;
}

pub trait PitchMod {
    const MOD: PitchTy = crate::MODULUS;
    type Output;

    fn pitchmod(&self) -> Self::Output;
}

impl<S> PitchMod for S
where
    S: PyMod<PitchTy>,
{
    type Output = <S as PyMod<PitchTy>>::Output;

    fn pitchmod(&self) -> Self::Output {
        self.pymod(Self::MOD)
    }
}

/*
 ************* Implementations *************
*/
use core::ops::{Add, Rem};
use num::traits::{Num, Signed};

impl<T> PyMod<T> for T
where
    T: Copy + Num + PartialOrd + Signed,
{
    type Output = T;

    fn pymod(&self, y: T) -> Self::Output {
        crate::pymod(*self, y)
    }
}

// impl<A, B, C> AbsMod<B> for A
// where
//     A: PyMod<B, Output = C>,
//     C: Signed,
// {
//     type Output = C;

//     fn absmod(&self, rhs: B) -> Self::Output {
//         self.pymod(rhs).abs()
//     }
// }

impl<A, B, C> AbsMod<B> for A
where
    A: Copy + Add<C, Output = C> + Rem<B, Output = C>,
    B: Copy,
    C: Add<B, Output = C> + Rem<B, Output = C> + Signed,
{
    type Output = C;

    fn absmod(&self, rhs: B) -> Self::Output {
        (((*self % rhs) + rhs) % rhs).abs()
    }
}
