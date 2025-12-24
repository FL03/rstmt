/*
    Appellation: num <module>
    Created At: 2025.12.20:06:02:11
    Contrib: @FL03
*/
use num_traits::{FromPrimitive, One, ToPrimitive, Zero};

pub trait ScalarNum
where
    Self: Clone
        + Default
        + PartialEq
        + PartialOrd
        + FromPrimitive
        + ToPrimitive
        + One
        + Zero
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
}

/*
 ************* Implementations *************
*/
impl<T> ScalarNum for T
where
    T: Clone
        + Default
        + PartialEq
        + PartialOrd
        + FromPrimitive
        + ToPrimitive
        + One
        + Zero
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
// macro_rules! impl_number {
//     (impl $trait:ident for { $($T:ty),* $(,)? }) => {
//         $(
//             impl $trait for $T {
//                 seal! {}
//             }
//         )*
//     };
// }

// impl_number! {
//     impl ScalarNum for {
//         u8, u16, u32, u64, u128, usize,
//         i8, i16, i32, i64, i128, isize,
//         f32, f64
//     }
// }
