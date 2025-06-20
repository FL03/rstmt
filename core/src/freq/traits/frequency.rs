/*
    appellation: frequency <module>
    authors: @FL03
*/

/// [`RawFrequency`] defines an interface for all raw Frequency types.
///
/// **note:** This trait is sealed and cannot be implemented outside of this crate.
pub trait RawFrequency: Send + Sync + core::fmt::Debug + core::fmt::Display {
    private!();
}
/// The [`FrequencyNum`] trait extends the [`RawFrequency`] trait with additional numeric operations
/// and traits.
pub trait FrequencyNum: RawFrequency
where
    Self: 'static
        + Copy
        + Eq
        + Default
        + PartialEq
        + PartialOrd
        + core::ops::Add<Output = Self>
        + core::ops::Sub<Output = Self>
        + core::ops::Mul<Output = Self>
        + core::ops::Div<Output = Self>
        + core::ops::Rem<Output = Self>
        + core::ops::Neg<Output = Self>
        + core::ops::Not<Output = Self>
        + core::ops::AddAssign
        + core::ops::SubAssign
        + core::ops::MulAssign
        + core::ops::DivAssign
        + core::ops::RemAssign
        + num_traits::NumRef
        + num_traits::FromPrimitive
        + num_traits::ToPrimitive
        + num_traits::Signed
        + num_traits::Zero
        + num_traits::One,
{
}

/*
 ************* Implementations *************
*/

macro_rules! impl_raw_frequency {
    ($($t:ty),* $(,)?) => {
        $(
            impl_raw_frequency!(@impl $t);
        )*
    };
    (@impl $t:ty) => {
        impl RawFrequency for $t {
            seal!();
        }
    };
}

impl_raw_frequency!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);

impl<T> FrequencyNum for T where
    T: RawFrequency
        + 'static
        + Copy
        + Default
        + Eq
        + PartialEq
        + PartialOrd
        + core::ops::Add<Output = Self>
        + core::ops::Sub<Output = Self>
        + core::ops::Mul<Output = Self>
        + core::ops::Div<Output = Self>
        + core::ops::Rem<Output = Self>
        + core::ops::Neg<Output = Self>
        + core::ops::Not<Output = Self>
        + core::ops::AddAssign
        + core::ops::SubAssign
        + core::ops::MulAssign
        + core::ops::DivAssign
        + core::ops::RemAssign
        + num_traits::NumRef
        + num_traits::FromPrimitive
        + num_traits::ToPrimitive
        + num_traits::Signed
        + num_traits::Zero
        + num_traits::One
{
}
