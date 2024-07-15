/*
    Appellation: pitch_ops <impl>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{Pitch, PitchMod, PitchTy, PyMod};
use num::{Num, One, Zero};

macro_rules! impl_interval_method {
    (@impl $trait:ident.$call:ident) => {
        paste::paste! {
            pub fn [<$call _interval>]<T>(&self, rhs: T) -> Self where i8: ::core::ops::$trait<T, Output = PitchTy> {
                let p = ::core::ops::$trait::$call(self.value(), rhs);
                Self::new(p)
            }
        }
    };
    ($($trait:ident.$call:ident),* $(,)?) => {
        $(
            impl_interval_method!(@impl $trait.$call);
        )*
    };
}

impl Pitch {
    impl_interval_method!(Add.add, Div.div, Mul.mul, Rem.rem, Sub.sub);
}

wrapper_ops!(Pitch::<PitchTy>: Add.add, Div.div, Mul.mul, Rem.rem, Sub.sub);

wrapper_unop!(Pitch impls Neg.neg, Not.not);

impl PitchMod for Pitch {
    type Output = Self;

    fn pitchmod(&self) -> Self::Output {
        Self(self.0.pymod(Self::MOD))
    }
}

impl One for Pitch {
    fn one() -> Self {
        Self(PitchTy::one())
    }
}

impl Zero for Pitch {
    fn zero() -> Self {
        Self(PitchTy::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl Num for Pitch {
    type FromStrRadixErr = <PitchTy as Num>::FromStrRadixErr;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        PitchTy::from_str_radix(s, radix).map(Self)
    }
}
