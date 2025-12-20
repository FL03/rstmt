/*
    Appellation: impl_pitch_ext <module>
    Created At: 2025.12.20:09:06:46
    Contrib: @FL03
*/
use super::Pitch;
use num_traits::{Num, One, Zero};

contained::fmt_wrapper! {
    impl Pitch<T> {
        Binary,
        Debug,
        Display,
        LowerExp,
        LowerHex,
        Octal,
        UpperExp,
        UpperHex
    }
}

contained::binary_wrapper! {
    impl Pitch {
        Add.add,
        Div.div,
        Mul.mul,
        Rem.rem,
        Sub.sub,
        BitAnd.bitand,
        BitOr.bitor,
        BitXor.bitxor,
        Shl.shl,
        Shr.shr
    }
}

contained::unary_wrapper! {
    impl Pitch {
        Neg.neg,
        Not.not
    }
}

impl<T> AsRef<T> for Pitch<T> {
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<T> AsMut<T> for Pitch<T> {
    fn as_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> core::borrow::Borrow<T> for Pitch<T> {
    fn borrow(&self) -> &T {
        self.get()
    }
}

impl<T> core::borrow::BorrowMut<T> for Pitch<T> {
    fn borrow_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> core::ops::Deref for Pitch<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> core::ops::DerefMut for Pitch<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<T> One for Pitch<T>
where
    T: One,
{
    fn one() -> Self {
        Pitch::new(T::one())
    }
}

impl<T> Zero for Pitch<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Pitch::new(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.get().is_zero()
    }
}

impl<T, E> Num for Pitch<T>
where
    T: Num<FromStrRadixErr = E>,
{
    type FromStrRadixErr = T::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(str, radix).map(Pitch::new)
    }
}

impl<T: PartialEq> PartialEq<T> for Pitch<T> {
    fn eq(&self, other: &T) -> bool {
        self.get() == other
    }
}

impl<T: PartialOrd> PartialOrd<T> for Pitch<T> {
    fn partial_cmp(&self, other: &T) -> Option<core::cmp::Ordering> {
        self.get().partial_cmp(other)
    }
}
