/*
    Appellation: num <module>
    Contrib: @FL03
*/
use num_traits::{FromPrimitive, Zero};

/// a functional implementation of python's modulo operator
fn _pymod<A, B, C>(lhs: A, rhs: B) -> C
where
    A: core::ops::Rem<B, Output = C>,
    B: Copy + Zero + PartialOrd,
    C: core::ops::Add<B, Output = C> + Zero + PartialOrd,
{
    let r = lhs % rhs;
    if (r < C::zero() && rhs > B::zero()) || (r > C::zero() && rhs < B::zero()) {
        r + rhs
    } else {
        r
    }
}

/// The [`PyMod`] trait defines a pythonic modulo operator that can be used to perform modulo
/// operations similar to Python's `%` operator, which behaves differently than Rust's `%`
/// operator when dealing with negative numbers.
pub trait PyMod<Rhs = Self> {
    type Output;

    fn pymod(self, rhs: Rhs) -> Self::Output;
}

/// this trait relies on a python modulo operator with a divisor of 12
pub trait PitchMod {
    type Output;

    fn pmod(self) -> Self::Output;
}

impl<A, B, C> PyMod<B> for A
where
    A: core::ops::Rem<B, Output = C>,
    B: Copy + PartialOrd + Zero,
    C: PartialOrd + Zero + core::ops::Add<B, Output = C>,
{
    type Output = C;

    fn pymod(self, rhs: B) -> Self::Output {
        _pymod(self, rhs)
    }
}

impl<A> PitchMod for A
where
    A: PyMod<A, Output = A> + FromPrimitive,
{
    type Output = A::Output;

    fn pmod(self) -> Self::Output {
        self.pymod(A::from_i32(12).unwrap())
    }
}
