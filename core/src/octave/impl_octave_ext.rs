/*
    Appellation: impl_octave_ext <module>
    Created At: 2025.12.21:08:48:17
    Contrib: @FL03
*/
use super::{Octave, RawOctave};
use num_traits::{Num, One, Zero};

contained::fmt_wrapper! {
    impl Octave<T> {
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

contained::binary_wrapper! {
    impl Octave {
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
    impl Octave {
        Neg.neg,
        Not.not,
    }
}

impl<T> One for Octave<T>
where
    T: One,
{
    fn one() -> Self {
        Octave(T::one())
    }
}

impl<T> Zero for Octave<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Octave(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<T> Num for Octave<T>
where
    T: Num,
{
    type FromStrRadixErr = T::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(str, radix).map(Octave)
    }
}

impl<T> AsRef<T> for Octave<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Octave<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> core::borrow::Borrow<T> for Octave<T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T> core::borrow::BorrowMut<T> for Octave<T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> core::ops::Deref for Octave<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> core::ops::DerefMut for Octave<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for Octave<T>
where
    T: RawOctave,
{
    fn from(index: T) -> Self {
        Octave(index)
    }
}

impl<T> PartialEq<T> for Octave<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        &self.0 == other
    }
}

impl<'a, T> PartialEq<&'a T> for Octave<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &&'a T) -> bool {
        &self.0 == *other
    }
}

impl<'a, T> PartialEq<&'a mut T> for Octave<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &&'a mut T) -> bool {
        &self.0 == *other
    }
}

impl<T> PartialOrd<T> for Octave<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &T) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<'a, T> PartialOrd<&'a T> for Octave<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &&'a T) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(*other)
    }
}

impl<'a, T> PartialOrd<&'a mut T> for Octave<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &&'a mut T) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(*other)
    }
}
