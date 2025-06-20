use crate::octave::Octave;
use num_traits::{Num, One, Zero};

impl<T> core::ops::Neg for Octave<T>
where
    T: core::ops::Neg,
{
    type Output = Octave<T::Output>;

    fn neg(self) -> Self::Output {
        self.map(|x| -x)
    }
}

impl<T> core::ops::Not for Octave<T>
where
    T: core::ops::Not,
{
    type Output = Octave<T::Output>;

    fn not(self) -> Self::Output {
        self.map(|x| !x)
    }
}

impl<T: One> One for Octave<T> {
    fn one() -> Self {
        Octave(T::one())
    }
}

impl<T: Zero> Zero for Octave<T> {
    fn zero() -> Self {
        Octave(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<T: Num> Num for Octave<T> {
    type FromStrRadixErr = T::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(str, radix).map(Octave)
    }
}

scsys::fmt_wrapper! {
    Octave<T>(
        Binary,
        Debug,
        Display,
        LowerExp,
        LowerHex,
        Octal,
        Pointer,
        UpperExp,
        UpperHex
    )
}

macro_rules! impl_assign_op {
    (@impl $trait:ident::$method:ident) => {
        impl<A, B> core::ops::$trait<Octave<B>> for Octave<A>
        where
            A: ::core::ops::$trait<B>
        {
            fn $method(&mut self, rhs: Octave<B>) {
                ::core::ops::$trait::$method(&mut self.0, rhs.0)
            }
        }

        impl<'a, A, B> core::ops::$trait<&'a Octave<B>> for Octave<A>
        where
            A: ::core::ops::$trait<&'a B>
        {
            fn $method(&mut self, rhs: &'a Octave<B>) {
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
        impl<A, B, C> ::core::ops::$trait<Octave<B>> for Octave<A>
        where
            A: ::core::ops::$trait<B, Output = C>
        {
            type Output = Octave<C>;

            fn $method(self, rhs: Octave<B>) -> Self::Output {
                Octave(::core::ops::$trait::$method(self.0, rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$trait<&'a Octave<B>> for Octave<A>
        where
            A: ::core::ops::$trait<&'a B, Output = C>
        {
            type Output = Octave<C>;

            fn $method(self, rhs: &'a Octave<B>) -> Self::Output {
                Octave(::core::ops::$trait::$method(self.0, &rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$trait<&'a Octave<B>> for &'a Octave<A>
        where
            &'a A: ::core::ops::$trait<&'a B, Output = C>
        {
            type Output = Octave<C>;

            fn $method(self, rhs: &'a Octave<B>) -> Self::Output {
                Octave(::core::ops::$trait::$method(&self.0, &rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$trait<Octave<B>> for &'a Octave<A>
        where
            &'a A: ::core::ops::$trait<B, Output = C>
        {
            type Output = Octave<C>;

            fn $method(self, rhs: Octave<B>) -> Self::Output {
                Octave(::core::ops::$trait::$method(&self.0, rhs.0))
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
