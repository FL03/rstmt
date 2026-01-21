/*
    Appellation: impl_scale_repr <module>
    Created At: 2026.01.20:15:39:15
    Contrib: @FL03
*/
use crate::compose::Scale;
use crate::freq::Frequency;

macro_rules! impl_scale_const {
    (@impl $t:ty) => {
        impl Scale<$t> {
            pub const A4: Frequency<$t> = Frequency(440 as $t);
        }
    };

    ($($t:ty),* $(,)?) => {
        $(impl_scale_const!(@impl $t);)*
    };
}

impl_scale_const! {
    f32, f64,
    u16, u32, u64, u128, usize,
    i16, i32, i64, i128, isize,
}
