/*
    Appellation: units <module>
    Created At: 2025.12.31:17:57:08
    Contrib: @FL03
*/
/// A musical unison is the identity interval in music theory, representing no pitch difference.
pub trait Unison {
    /// returns a single unison
    fn unison() -> Self;
    /// returns true if the caller's value is equivalent to a single unison.
    fn is_unison(&self) -> bool
    where
        Self: Sized + PartialEq,
    {
        *self == Self::unison()
    }
}
/// Similar to the [`One`](num_traits::One) trait, the [`Semitone`] is an identity and unit for
/// musical contexts.
pub trait Semitone {
    /// returns a single semitone
    fn semitone() -> Self;
    /// returns true if the caller's value is equivalent to a single semitone.
    fn is_semitone(&self) -> bool
    where
        Self: Sized + PartialEq,
    {
        *self == Self::semitone()
    }
}

pub trait WholeTone {
    /// returns a new instance set to the value of a whole tone (2 semitones)
    fn tone() -> Self;
}

/*
 ************* Implementations *************
*/
macro_rules! impl_semitone {
    ($($T:ty),* $(,)?) => {
        $(impl_semitone! { @impl $T })*
    };
    (@impl $T:ty) => {
        impl $crate::intervals::traits::Unison for $T {
            fn unison() -> Self {
                0 as $T
            }
        }

        impl $crate::intervals::traits::Semitone for $T {
            fn semitone() -> Self {
                1 as $T
            }
        }

        impl $crate::intervals::traits::WholeTone for $T {
            fn tone() -> Self {
                2 as $T
            }
        }
    };
}

impl_semitone! {
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64,
}
