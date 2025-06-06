/*
    Appellation: octave <types>
    Contrib: @FL03
*/

/// A type defining an octave
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Deserialize, serde_derive::Serialize),
    serde(default, transparent)
)]
#[repr(transparent)]
pub struct Octave<T = isize>(pub T);

impl<T> Octave<T> {
    /// returns a new instance initialized using the default value of the type
    pub fn new() -> Self
    where
        T: Default,
    {
        Octave(T::default())
    }
    /// returns a new instance of the [`Octave`] wrapping the given value
    pub const fn from_value(index: T) -> Self {
        Octave(index)
    }
    /// returns a new Octave with the value of one
    pub fn one() -> Self
    where
        T: num_traits::One,
    {
        Octave(T::one())
    }
    /// returns a new Octave with the value of zero
    pub fn zero() -> Self
    where
        T: num_traits::Zero,
    {
        Octave(T::zero())
    }
    /// returns a pointer to the inner value
    pub const fn as_ptr(&self) -> *const T {
        core::ptr::from_ref(&self.0)
    }
    /// returns a mutable pointer to the inner value
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        core::ptr::from_mut(&mut self.0)
    }
    /// consumes the index returning the inner value
    pub fn value(self) -> T {
        self.0
    }
    /// returns an immutable reference to the inner value
    pub const fn get(&self) -> &T {
        &self.0
    }
    /// returns a mutable reference to the inner value
    pub const fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
    /// apply a function to the inner value and returns a new Octave wrapping the result
    pub fn map<U, F>(self, f: F) -> Octave<U>
    where
        F: FnOnce(T) -> U,
    {
        Octave(f(self.0))
    }
    /// replaces the inner value with the given one and returns the old value
    pub const fn replace(&mut self, index: T) -> T {
        core::mem::replace(self.get_mut(), index)
    }
    /// set the index to the given value
    pub fn set(&mut self, index: T) -> &mut Self {
        *self.get_mut() = index;
        self
    }
    /// swap the values of two indices
    pub const fn swap(&mut self, other: &mut Self) {
        core::mem::swap(self.get_mut(), other.get_mut());
    }
    /// consumes the current instance to create another with the given value
    pub fn with<U>(self, other: U) -> Octave<U> {
        Octave(other)
    }
    /// takes and returns the inner value, replacing it with the logical [`default`](Default)
    /// of the type `T`
    pub fn take(&mut self) -> T
    where
        T: Default,
    {
        core::mem::take(self.get_mut())
    }
    /// returns a new instance with a clone of the inner value
    pub fn cloned(&self) -> Octave<T>
    where
        T: Clone,
    {
        Octave(self.get().clone())
    }
    /// returns a new instance with a copy of the inner value
    pub fn copied(&self) -> Octave<T>
    where
        T: Copy,
    {
        Octave(*self.get())
    }
    /// returns a new instance containing a reference to the inner value
    pub const fn view(&self) -> Octave<&T> {
        Octave(self.get())
    }
    /// returns a new instance containing a mutable reference to the inner value
    pub fn view_mut(&mut self) -> Octave<&mut T> {
        Octave(self.get_mut())
    }
}

#[cfg(feature = "rand")]
impl<T> Octave<T>
where
    rand_distr::StandardUniform: rand_distr::Distribution<T>,
{
    pub fn random() -> Self {
        Octave(rand::random())
    }
    pub fn random_in<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        Octave(rng.random())
    }
}

#[cfg(feature = "rand")]
impl<T> rand_distr::Distribution<Octave<T>> for rand_distr::StandardUniform
where
    rand_distr::StandardUniform: rand_distr::Distribution<T>,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Octave<T> {
        Octave(rng.random())
    }
}

impl<T> From<T> for Octave<T> {
    fn from(index: T) -> Self {
        Octave(index)
    }
}

impl<T> PartialEq<T> for Octave<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        self.get() == other
    }
}

impl<T> PartialOrd<T> for Octave<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &T) -> Option<core::cmp::Ordering> {
        self.get().partial_cmp(other)
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

impl<T> core::ops::Neg for Octave<T>
where
    T: core::ops::Neg,
{
    type Output = Octave<T::Output>;

    fn neg(self) -> Self::Output {
        Octave(-self.value())
    }
}

impl<T> core::ops::Not for Octave<T>
where
    T: core::ops::Not,
{
    type Output = Octave<T::Output>;

    fn not(self) -> Self::Output {
        Octave(!self.value())
    }
}

impl<T> num::One for Octave<T>
where
    T: num::One,
{
    fn one() -> Self {
        Octave(T::one())
    }
}

impl<T> num::Zero for Octave<T>
where
    T: num::Zero,
{
    fn zero() -> Self {
        Octave(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<T> num::Num for Octave<T>
where
    T: num::Num,
{
    type FromStrRadixErr = T::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(str, radix).map(Octave)
    }
}

scsys::fmt_wrapper! {
    Octave<T>(Binary, Debug, Display, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex)
}

macro_rules! impl_assign_op {
    (@impl $trait:ident::$method:ident) => {
        impl<A, B> core::ops::$trait<Octave<B>> for Octave<A>
        where
            A: ::core::ops::$trait<B>
        {
            fn $method(&mut self, rhs: Octave<B>) {
                ::core::ops::$trait::$method(self.get_mut(), rhs.value())
            }
        }

        impl<'a, A, B> core::ops::$trait<&'a Octave<B>> for Octave<A>
        where
            A: ::core::ops::$trait<&'a B>
        {
            fn $method(&mut self, rhs: &'a Octave<B>) {
                ::core::ops::$trait::$method(self.get_mut(), rhs.get())
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
                Octave(::core::ops::$trait::$method(self.value(), rhs.value()))
            }
        }

        impl<'a, A, B, C> ::core::ops::$trait<&'a Octave<B>> for Octave<A>
        where
            A: ::core::ops::$trait<&'a B, Output = C>
        {
            type Output = Octave<C>;

            fn $method(self, rhs: &'a Octave<B>) -> Self::Output {
                Octave(::core::ops::$trait::$method(self.value(), rhs.get()))
            }
        }

        impl<'a, A, B, C> ::core::ops::$trait<&'a Octave<B>> for &'a Octave<A>
        where
            &'a A: ::core::ops::$trait<&'a B, Output = C>
        {
            type Output = Octave<C>;

            fn $method(self, rhs: &'a Octave<B>) -> Self::Output {
                Octave(::core::ops::$trait::$method(self.get(), rhs.get()))
            }
        }

        impl<'a, A, B, C> ::core::ops::$trait<Octave<B>> for &'a Octave<A>
        where
            &'a A: ::core::ops::$trait<B, Output = C>
        {
            type Output = Octave<C>;

            fn $method(self, rhs: Octave<B>) -> Self::Output {
                Octave(::core::ops::$trait::$method(self.get(), rhs.value()))
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
