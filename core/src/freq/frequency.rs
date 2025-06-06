/*
    Appellation: frequency <module>
    Contrib: @FL03
*/
use super::RawFrequency;

/// The [`Frequency`] type is a generic wrapper around type `T` that implements the
/// [`RawFrequency`] trait. This implementation is designed to provide a consistent interface
/// for dealing with frequencies within the crate, enabling conversion, arithmetic operations,
/// and other utilities that are common to frequency values.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Deserialize, serde_derive::Serialize),
    serde(default, transparent)
)]
#[repr(transparent)]
pub struct Frequency<T = f64>(pub T);

impl<T> Frequency<T>
where
    T: RawFrequency,
{
    /// returns a new instance of the [`Frequency`] wrapping the given value
    pub const fn new(index: T) -> Self {
        Frequency(index)
    }
    /// returns a new instance initialized using the default value of the type
    pub fn default() -> Self
    where
        T: Default,
    {
        Frequency::new(T::default())
    }
    /// returns a new Frequency with the value of one
    pub fn one() -> Self
    where
        T: num_traits::One,
    {
        Frequency::new(T::one())
    }
    /// returns a new Frequency with the value of zero
    pub fn zero() -> Self
    where
        T: num_traits::Zero,
    {
        Frequency::new(T::zero())
    }
    /// returns a pointer to the inner value
    pub const fn as_ptr(&self) -> *const T {
        core::ptr::from_ref(self.get())
    }
    /// returns a mutable pointer to the inner value
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        core::ptr::from_mut(self.get_mut())
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
    /// apply a function to the inner value and returns a new Frequency wrapping the result
    pub fn map<U, F>(self, f: F) -> Frequency<U>
    where
        F: FnOnce(T) -> U,
    {
        Frequency(f(self.value()))
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
    pub fn with<U>(self, other: U) -> Frequency<U> {
        Frequency(other)
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
    pub fn cloned(&self) -> Frequency<T>
    where
        T: Clone,
    {
        Frequency(self.get().clone())
    }
    /// returns a new instance with a copy of the inner value
    pub fn copied(&self) -> Frequency<T>
    where
        T: Copy,
    {
        Frequency(*self.get())
    }
    /// returns a new instance containing a reference to the inner value
    pub const fn view(&self) -> Frequency<&T> {
        Frequency(self.get())
    }
    /// returns a new instance containing a mutable reference to the inner value
    pub fn view_mut(&mut self) -> Frequency<&mut T> {
        Frequency(self.get_mut())
    }
}

impl<T> PartialEq<T> for Frequency<T>
where
    T: RawFrequency + PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        self.get() == other
    }
}

impl<T> PartialOrd<T> for Frequency<T>
where
    T: RawFrequency + PartialOrd,
{
    fn partial_cmp(&self, other: &T) -> Option<core::cmp::Ordering> {
        self.get().partial_cmp(other)
    }
}

#[cfg(feature = "rand")]
impl<T> Frequency<T>
where
    T: RawFrequency,
    rand_distr::StandardUniform: rand_distr::Distribution<T>,
{
    pub fn random() -> Self {
        Frequency(rand::random())
    }
    pub fn random_in<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        Frequency(rng.random())
    }
}

#[cfg(feature = "rand")]
impl<T> rand_distr::Distribution<Frequency<T>> for rand_distr::StandardUniform
where
    T: RawFrequency,
    rand_distr::StandardUniform: rand_distr::Distribution<T>,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Frequency<T> {
        Frequency(rng.random())
    }
}

impl<T> AsRef<T> for Frequency<T>
where
    T: RawFrequency,
{
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<T> AsMut<T> for Frequency<T>
where
    T: RawFrequency,
{
    fn as_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> core::borrow::Borrow<T> for Frequency<T>
where
    T: RawFrequency,
{
    fn borrow(&self) -> &T {
        self.get()
    }
}

impl<T> core::borrow::BorrowMut<T> for Frequency<T>
where
    T: RawFrequency,
{
    fn borrow_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> core::ops::Deref for Frequency<T>
where
    T: RawFrequency,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> core::ops::DerefMut for Frequency<T>
where
    T: RawFrequency,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<T> core::ops::Neg for Frequency<T>
where
    T: RawFrequency + core::ops::Neg,
{
    type Output = Frequency<<T as core::ops::Neg>::Output>;

    fn neg(self) -> Self::Output {
        Frequency(-self.value())
    }
}

impl<T> core::ops::Not for Frequency<T>
where
    T: RawFrequency + core::ops::Not,
{
    type Output = Frequency<<T as core::ops::Not>::Output>;

    fn not(self) -> Self::Output {
        Frequency(!self.value())
    }
}

impl<T> num_traits::One for Frequency<T>
where
    T: RawFrequency + num_traits::One,
{
    fn one() -> Self {
        Frequency(T::one())
    }
}

impl<T> num_traits::Zero for Frequency<T>
where
    T: RawFrequency + num_traits::Zero,
{
    fn zero() -> Self {
        Frequency(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.get().is_zero()
    }
}

impl<T> num_traits::Num for Frequency<T>
where
    T: RawFrequency + num_traits::Num,
{
    type FromStrRadixErr = T::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(str, radix).map(Frequency)
    }
}

scsys::fmt_wrapper! {
    Frequency<T>(Binary, Debug, Display, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex)
}

macro_rules! impl_assign_op {
    (@impl $trait:ident::$method:ident) => {
        impl<A, B> ::core::ops::$trait<Frequency<B>> for Frequency<A>
        where
            A: RawFrequency + ::core::ops::$trait<B>,
            B: RawFrequency,
        {
            fn $method(&mut self, rhs: Frequency<B>) {
                ::core::ops::$trait::$method(self.get_mut(), rhs.value())
            }
        }

        impl<'a, A, B> ::core::ops::$trait<&'a Frequency<B>> for Frequency<A>
        where
            A: RawFrequency + ::core::ops::$trait<&'a B>,
            B: RawFrequency,
        {
            fn $method(&mut self, rhs: &'a Frequency<B>) {
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
        impl<A, B, C> ::core::ops::$trait<Frequency<B>> for Frequency<A>
        where
            A: RawFrequency + ::core::ops::$trait<B, Output = C>,
            B: RawFrequency,
            C: RawFrequency,
        {
            type Output = Frequency<C>;

            fn $method(self, rhs: Frequency<B>) -> Self::Output {
                Frequency(::core::ops::$trait::$method(self.value(), rhs.value()))
            }
        }

        impl<'a, A, B, C> ::core::ops::$trait<&'a Frequency<B>> for Frequency<A>
        where
            A: ::core::ops::$trait<&'a B, Output = C>,
            A: RawFrequency,
            B: RawFrequency,
            C: RawFrequency,
        {
            type Output = Frequency<C>;

            fn $method(self, rhs: &'a Frequency<B>) -> Self::Output {
                Frequency(::core::ops::$trait::$method(self.value(), rhs.get()))
            }
        }

        impl<'a, A, B, C> ::core::ops::$trait<&'a Frequency<B>> for &'a Frequency<A>
        where
            &'a A: ::core::ops::$trait<&'a B, Output = C>,
            A: RawFrequency,
            B: RawFrequency,
            C: RawFrequency,
        {
            type Output = Frequency<C>;

            fn $method(self, rhs: &'a Frequency<B>) -> Self::Output {
                Frequency(::core::ops::$trait::$method(self.get(), rhs.get()))
            }
        }

        impl<'a, A, B, C> ::core::ops::$trait<Frequency<B>> for &'a Frequency<A>
        where
            &'a A: ::core::ops::$trait<B, Output = C>,
            A: RawFrequency,
            B: RawFrequency,
            C: RawFrequency,
        {
            type Output = Frequency<C>;

            fn $method(self, rhs: Frequency<B>) -> Self::Output {
                Frequency(::core::ops::$trait::$method(self.get(), rhs.value()))
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
