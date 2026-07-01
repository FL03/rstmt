/*
    Appellation: raw_frequency <module>
    Created At: 2026.01.20:08:32:55
    Contrib: @FL03
*/

/// [`RawFrequency`] is a marker trait denoting objects capable of representing a frequency
pub trait RawFrequency {
    private! {}
}

/*
 ************* Implementations *************
*/
macro_rules! raw_frequency {
    (@impl $T:ty) => {
        impl RawFrequency for $T {
            seal! {}
        }
    };
    {$($T:ty),* $(,)?} => {
        $(raw_frequency!(@impl $T);)*
    };
}

raw_frequency! {
    f32, f64,
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize
}

#[cfg(feature = "complex")]
impl<T> RawFrequency for num_complex::Complex<T>
where
    T: RawFrequency,
{
    seal! {}
}
