/*
    appellation: impl_freq_ops <module>
    authors: @FL03
*/
use crate::freq::{Frequency, RawFrequency, utils};
use num_traits::{Float, FromPrimitive, Num, One, Zero};

impl<T> Frequency<T>
where
    T: RawFrequency + Float + FromPrimitive,
{
    /// calculate the frequency (in hertz) of a given pitch class, using the formula:
    ///
    /// ```math
    /// f = base * 2^(n/12)
    /// ```
    pub fn compute_freq_from_scale(n: isize, base: Option<T>) -> Option<Self> {
        utils::compute_freq_from_scale(n, base).map(Frequency)
    }
    /// Compute the pitch class of a frequency (in hertz), using the formula:
    ///
    /// ```math
    /// n = 12 * log2(f / base)
    /// ```
    pub fn get_scale_of_freq<U>(&self, base: Option<T>) -> Option<isize> {
        // Ensure frequency is positive
        if self.get() <= &T::zero() {
            return None;
        }
        let freq = *self.get();
        utils::get_scale_of_freq(freq, base)
    }
}

impl<T> core::ops::Neg for Frequency<T>
where
    T: core::ops::Neg,
{
    type Output = Frequency<T::Output>;

    fn neg(self) -> Self::Output {
        Frequency(-self.0)
    }
}

impl<T> core::ops::Not for Frequency<T>
where
    T: core::ops::Not,
{
    type Output = Frequency<T::Output>;

    fn not(self) -> Self::Output {
        Frequency(!self.0)
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

macro_rules! impl_assign_op {
    (@impl $trait:ident::$method:ident) => {
        impl<A, B> ::core::ops::$trait<Frequency<B>> for Frequency<A>
        where
            A: ::core::ops::$trait<B>,
        {
            fn $method(&mut self, rhs: Frequency<B>) {
                ::core::ops::$trait::$method(&mut self.0, rhs.0)
            }
        }

        impl<'a, A, B> ::core::ops::$trait<&'a Frequency<B>> for Frequency<A>
        where
            A: ::core::ops::$trait<&'a B>,
        {
            fn $method(&mut self, rhs: &'a Frequency<B>) {
                ::core::ops::$trait::$method(&mut self.0, &rhs.0)
            }
        }
    };

    ($($trait:ident::$method:ident),* $(,)?) => {
        $(
            impl_assign_op!(@impl $trait::$method);
        )*
    };
}

macro_rules! impl_bin_op {
    (@impl $trait:ident::$method:ident) => {
        impl<A, B, C> ::core::ops::$trait<Frequency<B>> for Frequency<A>
        where
            A: ::core::ops::$trait<B, Output = C>,
        {
            type Output = Frequency<C>;

            fn $method(self, rhs: Frequency<B>) -> Self::Output {
                Frequency(::core::ops::$trait::$method(self.0, rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$trait<&'a Frequency<B>> for Frequency<A>
        where
            A: ::core::ops::$trait<&'a B, Output = C>,
        {
            type Output = Frequency<C>;

            fn $method(self, rhs: &'a Frequency<B>) -> Self::Output {
                Frequency(::core::ops::$trait::$method(self.0, &rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$trait<&'a mut Frequency<B>> for Frequency<A>
        where
            A: ::core::ops::$trait<&'a B, Output = C>,
        {
            type Output = Frequency<C>;

            fn $method(self, rhs: &'a mut Frequency<B>) -> Self::Output {
                Frequency(::core::ops::$trait::$method(self.0, &rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$trait<&'a Frequency<B>> for &'a Frequency<A>
        where
            &'a A: ::core::ops::$trait<&'a B, Output = C>,
        {
            type Output = Frequency<C>;

            fn $method(self, rhs: &'a Frequency<B>) -> Self::Output {
                Frequency(::core::ops::$trait::$method(&self.0, &rhs.0))
            }
        }

        impl<A, B, C> ::core::ops::$trait<Frequency<B>> for &Frequency<A>
        where
            for<'a> &'a A: ::core::ops::$trait<B, Output = C>,
        {
            type Output = Frequency<C>;

            fn $method(self, rhs: Frequency<B>) -> Self::Output {
                Frequency(::core::ops::$trait::$method(&self.0, rhs.0))
            }
        }

        impl<A, B, C> ::core::ops::$trait<Frequency<B>> for &mut Frequency<A>
        where
        for<'a> &'a A: ::core::ops::$trait<B, Output = C>,
        {
            type Output = Frequency<C>;

            fn $method(self, rhs: Frequency<B>) -> Self::Output {
                Frequency(::core::ops::$trait::$method(&self.0, rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$trait<&'a mut Frequency<B>> for &'a mut Frequency<A>
        where
            &'a A: ::core::ops::$trait<&'a B, Output = C>,
        {
            type Output = Frequency<C>;

            fn $method(self, rhs: &'a mut Frequency<B>) -> Self::Output {
                Frequency(::core::ops::$trait::$method(&self.0, &rhs.0))
            }
        }
    };

    ($($trait:ident::$method:ident),* $(,)?) => {

        $(
            impl_bin_op!(@impl $trait::$method);
        )*

    };
}

impl_assign_op! {
    AddAssign::add_assign,
    SubAssign::sub_assign,
    MulAssign::mul_assign,
    DivAssign::div_assign,
    RemAssign::rem_assign,
    BitAndAssign::bitand_assign,
    BitOrAssign::bitor_assign,
    BitXorAssign::bitxor_assign,
    ShlAssign::shl_assign,
    ShrAssign::shr_assign,
}

impl_bin_op! {
    Add::add,
    Sub::sub,
    Mul::mul,
    Div::div,
    Rem::rem,
    BitAnd::bitand,
    BitOr::bitor,
    BitXor::bitxor,
    Shl::shl,
    Shr::shr,
}
