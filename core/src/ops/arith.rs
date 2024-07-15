/*
    Appellation: arith <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Floor {
    type Output;

    fn floor(self) -> Self::Output;
}

pub trait FloorDiv<Rhs = Self> {
    type Output;

    fn floor_div(self, rhs: Rhs) -> Self::Output;
}


macro_rules! impl_floor {
    (@base $t:ty) => {
        impl Floor for $t {
            type Output = $t;

            fn floor(self) -> Self::Output {
                <$t>::floor(self)
            }
        }
    };
    (@impl f32) => {
        impl_floor!(@base f32);
    };
    (@impl f64) => {
        impl_floor!(@base f64);
    };
    (@impl $t:ty) => {
        impl Floor for $t {
            type Output = $t;

            fn floor(self) -> Self::Output {
                self
            }
        }
    };
    ($($t:ty),*) => {
        $(
            impl_floor!(@impl $t);
        )*
    };
}

macro_rules! impl_floor_div {
    (@float $t:ty) => {
        impl<T, O> FloorDiv<T> for $t where $t: ::core::ops::Div<T, Output = O> {
            type Output = O;

            fn floor_div(self, rhs: T) -> Self::Output {
                (self / rhs).floor()
            }
        }
    };
    (@impl f32) => {
        impl_floor_div!(@float f32);
    };
    (@impl f64) => {
        impl_floor_div!(@float f64);
    };
    (@impl $t:ty) => {
        impl<T, O> FloorDiv<T> for $t where $t: ::core::ops::Div<T, Output = O> {
            type Output = O;

            fn floor_div(self, rhs: T) -> Self::Output {
                self / rhs
            }
        }
    };
    ($($t:ty),*) => {
        $(
            impl_floor_div!(@impl $t);
        )*
    };
}

impl_floor!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
impl_floor_div!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);