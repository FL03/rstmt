/*
    Appellation: num <module>
    Created At: 2025.12.20:06:02:11
    Contrib: @FL03
*/
use num_traits::{FromPrimitive, One, ToPrimitive, Zero};

pub trait OrderedNum
where
    Self: PartialEq + PartialOrd + One + Zero,
{
    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }

    fn is_one(&self) -> bool {
        *self == Self::one()
    }

    fn is_positive(&self) -> bool {
        *self > Self::zero()
    }

    fn is_negative(&self) -> bool {
        *self < Self::zero()
    }
}

pub trait Numerical
where
    Self: Clone
        + Copy
        + Default
        + PartialEq
        + PartialOrd
        + FromPrimitive
        + ToPrimitive
        + One
        + Zero
        + core::fmt::Debug
        + core::fmt::Display
        + core::ops::Add<Output = Self>
        + core::ops::Sub<Output = Self>
        + core::ops::Mul<Output = Self>
        + core::ops::Div<Output = Self>
        + core::ops::Rem<Output = Self>
        + core::ops::AddAssign
        + core::ops::DivAssign
        + core::ops::MulAssign
        + core::ops::RemAssign
        + core::ops::SubAssign,
{
    private! {}

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }

    fn is_one(&self) -> bool {
        *self == Self::one()
    }

    fn is_positive(&self) -> bool {
        *self > Self::zero()
    }

    fn is_negative(&self) -> bool {
        *self < Self::zero()
    }

    fn apply<F, U>(self, f: F) -> U
    where
        F: FnOnce(Self) -> U,
    {
        f(self)
    }

    fn abs(self) -> Self
    where
        Self: core::ops::Neg<Output = Self>,
    {
        if self.is_negative() { -self } else { self }
    }
}

pub trait MusicScalar
where
    Self: Numerical + crate::PyMod<Output = Self> + crate::PitchMod<Output = Self>,
{
    private! {}
}
/*
 ************* Implementations *************
*/

impl<T> OrderedNum for T
where
    T: PartialEq + PartialOrd + One + Zero,
{
}

impl<T> Numerical for T
where
    T: Clone
        + Copy
        + Default
        + PartialEq
        + PartialOrd
        + FromPrimitive
        + ToPrimitive
        + One
        + Zero
        + core::fmt::Debug
        + core::fmt::Display
        + core::ops::Add<Output = Self>
        + core::ops::Sub<Output = Self>
        + core::ops::Mul<Output = Self>
        + core::ops::Div<Output = Self>
        + core::ops::Rem<Output = Self>
        + core::ops::AddAssign
        + core::ops::DivAssign
        + core::ops::MulAssign
        + core::ops::RemAssign
        + core::ops::SubAssign,
{
    seal! {}
}

impl<T> MusicScalar for T
where
    T: Numerical + crate::PyMod<Output = Self> + crate::PitchMod<Output = Self>,
{
    seal! {}
}

// macro_rules! numerical {
//     (impl $trait:ident for { $($T:ty),* $(,)? }) => {
//         $(impl $trait for $T {})*
//     };
// }

// numerical! {
//     impl Numerical for {
//         u8, u16, u32, u64, u128, usize,
//         i8, i16, i32, i64, i128, isize,
//         f32, f64
//     }
// }
