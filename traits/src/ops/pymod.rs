/*
    Appellation: num <module>
    Contrib: @FL03
*/
use crate::OrderedNum;
use num_traits::{FromPrimitive, Zero};

/// a functional implementation of python's modulo operator
fn _pymod<A, B, C>(lhs: A, rhs: B) -> C
where
    B: PartialOrd + Zero,
    C: PartialOrd + Zero + core::ops::Add<B, Output = C>,
    for<'a> A: core::ops::Rem<&'a B, Output = C>,
{
    let r = lhs % &rhs;
    if (r < <C>::zero() && rhs > <B>::zero()) || (r > <C>::zero() && rhs < <B>::zero()) {
        r + rhs
    } else {
        r
    }
}

/// [`PyMod`] is a modulo operator inspired by Python's `%` operator, which handles negative
/// values differently than rust's built-in `%` operator.
pub trait PyMod<Rhs = Self> {
    type Output;

    fn pymod(self, rhs: Rhs) -> Self::Output;
}

/// The [`PitchMod`] trait is a particular implementation of the [`PyMod`] trait, specifically
/// binding the caller to a mod 12 space.
pub trait PitchMod {
    type Output;

    fn pmod(self) -> Self::Output;
}

/*
 ************* Implementations *************
*/
impl<A, B, C> PyMod<B> for A
where
    B: OrderedNum,
    C: OrderedNum + core::ops::Add<B, Output = C>,
    for<'a> A: core::ops::Rem<&'a B, Output = C>,
{
    type Output = C;

    fn pymod(self, rhs: B) -> Self::Output {
        let r = self % &rhs;
        if (r.is_negative() && rhs.is_positive()) || (r.is_positive() && rhs.is_negative()) {
            r + rhs
        } else {
            r
        }
    }
}

impl<A> PitchMod for A
where
    A: PyMod<A, Output = A> + FromPrimitive,
{
    type Output = A::Output;

    fn pmod(self) -> Self::Output {
        self.pymod(A::from_u8(12).unwrap())
    }
}
