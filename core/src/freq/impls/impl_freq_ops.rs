/*
    appellation: impl_freq_ops <module>
    authors: @FL03
*/
use crate::freq::{Frequency, classify_freq_by_scale, get_freq_from_scale};
use num_traits::{Float, FromPrimitive, Num, One, Zero};
use rstmt_traits::ClassifyBy;

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

impl<T> Frequency<T>
where
    T: Float + FromPrimitive,
{
    /// calculate the frequency (in hertz) of a given pitch class, using the formula:
    ///
    /// ```math
    /// F=\gamma\cdot{2^\frac{n}{12}}
    /// ```
    pub fn compute_freq_from_scale(n: isize, base: Option<T>) -> Option<Self> {
        get_freq_from_scale(n, base).map(Frequency)
    }
    /// Compute the pitch class of a frequency (in hertz), using the formula:
    ///
    /// ```math
    /// n = 12\cdot\log_2(\frac{F}{\gamma})
    /// ```
    pub fn classify_by(&self, base: Option<T>) -> Option<isize> {
        classify_freq_by_scale(**self, base)
    }
}

impl<T> ClassifyBy<T> for Frequency<T>
where
    T: Float + FromPrimitive,
{
    type Output = Option<isize>;

    fn classify_by(&self, base: T) -> Self::Output {
        self.classify_by(Some(base))
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
