/*
    Appellation: impl_freq_ext <module>
    Created At: 2025.12.29:17:36:58
    Contrib: @FL03
*/
use crate::freq::RawFrequency;
use crate::freq::frequency::Frequency;
use num_traits::{Num, One, Zero};

contained::fmt_wrapper! {
    impl Frequency<T> {
        Binary,
        Debug,
        Display,
        LowerExp,
        LowerHex,
        Octal,
        Pointer,
        UpperExp,
        UpperHex
    }
}

contained::unary_wrapper! {
    impl Frequency {
        Neg.neg,
        Not.not,
    }
}

contained::binary_wrapper! {
    impl Frequency {
        Add.add,
        Sub.sub,
        Mul.mul,
        Div.div,
        Rem.rem,
        BitAnd.bitand,
        BitOr.bitor,
        BitXor.bitxor,
        Shl.shl,
        Shr.shr,
    }
}

impl<T> AsRef<T> for Frequency<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Frequency<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> core::borrow::Borrow<T> for Frequency<T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T> core::borrow::BorrowMut<T> for Frequency<T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> core::ops::Deref for Frequency<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> core::ops::DerefMut for Frequency<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for Frequency<T>
where
    T: RawFrequency,
{
    fn from(value: T) -> Self {
        Frequency(value)
    }
}

impl<T> PartialEq<T> for Frequency<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        &self.0 == other
    }
}

impl<'a, T> PartialEq<&'a T> for Frequency<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &&'a T) -> bool {
        &self.0 == *other
    }
}

impl<'a, T> PartialEq<&'a mut T> for Frequency<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &&'a mut T) -> bool {
        &self.0 == *other
    }
}

impl<T> PartialOrd<T> for Frequency<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &T) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<'a, T> PartialOrd<&'a T> for Frequency<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &&'a T) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(*other)
    }
}

impl<'a, T> PartialOrd<&'a mut T> for Frequency<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &&'a mut T) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(*other)
    }
}

impl<T> One for Frequency<T>
where
    T: One,
{
    fn one() -> Self {
        Frequency(T::one())
    }
}

impl<T> Zero for Frequency<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Frequency(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<T> Num for Frequency<T>
where
    T: Num,
{
    type FromStrRadixErr = T::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(str, radix).map(Frequency)
    }
}
