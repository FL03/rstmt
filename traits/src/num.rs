/*
    Appellation: num <module>
    Created At: 2025.12.20:06:02:11
    Contrib: @FL03
*/
use num_traits::{FromPrimitive, One, ToPrimitive, Zero};

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

impl<T> MusicScalar for T
where
    T: Numerical + crate::PyMod<Output = Self> + crate::PitchMod<Output = Self>,
{
    seal! {}
}

macro_rules! numerical {
    (impl $trait:ident for { $($T:ty),* $(,)? }) => {
        $(impl $trait for $T {})*
    };
}

numerical! {
    impl Numerical for {
        u8, u16, u32, u64, u128, usize,
        i8, i16, i32, i64, i128, isize,
        f32, f64
    }
}
