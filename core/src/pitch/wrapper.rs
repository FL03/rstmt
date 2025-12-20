/*
    Appellation: pitch <module>
    Contrib: @FL03
*/
use super::RawPitch;

/// Musically, a pitch is defined to be a discrete frequency that may be symbolically
/// represented via a pitch class.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Deserialize, serde_derive::Serialize),
    serde(default, transparent)
)]
#[repr(transparent)]
pub struct Pitch<T = f64> {
    pub(crate) value: T,
}

impl<T> Pitch<T>
where
    T: RawPitch,
{
    /// returns a new instance of the [`Pitch`] wrapping the given value
    pub const fn new(value: T) -> Self {
        Pitch { value }
    }
    /// initialize a new instance of the pitch using the result of the given function
    pub fn init<F>(f: F) -> Self
    where
        F: FnOnce() -> T,
    {
        Pitch::new(f())
    }
    /// returns a new Pitch with the value of one
    pub fn one() -> Self
    where
        T: num_traits::One,
    {
        Pitch::init(T::one)
    }
    /// returns a new Pitch with the value of zero
    pub fn zero() -> Self
    where
        T: num_traits::Zero,
    {
        Pitch::init(T::zero)
    }
    /// returns a pointer to the inner value
    pub const fn as_ptr(&self) -> *const T {
        core::ptr::from_ref(self.get())
    }
    /// returns a mutable pointer to the inner value
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        core::ptr::from_mut(self.get_mut())
    }
    #[inline]
    /// consumes the index returning the inner value
    pub fn value(self) -> T {
        self.value
    }
    /// returns an immutable reference to the inner value
    pub const fn get(&self) -> &T {
        &self.value
    }
    /// returns a mutable reference to the inner value
    pub const fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }
    /// apply a function to the inner value and returns a new Pitch wrapping the result
    pub fn map<U, F>(self, f: F) -> Pitch<U>
    where
        F: FnOnce(T) -> U,
    {
        Pitch {
            value: f(self.value()),
        }
    }
    /// applies the function onto a mutable reference of the inner value
    pub fn apply_mut<F>(&mut self, mut f: F) -> &mut Self
    where
        F: FnMut(&mut T),
    {
        f(self.get_mut());
        self
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
    pub fn with<U>(self, value: U) -> Pitch<U> {
        Pitch { value }
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
    pub fn cloned(&self) -> Pitch<T>
    where
        T: Clone,
    {
        Pitch::new(self.get().clone())
    }
    /// returns a new instance with a copy of the inner value
    pub fn copied(&self) -> Pitch<T>
    where
        T: Copy,
    {
        Pitch::new(*self.get())
    }
    /// returns a new instance containing a reference to the inner value
    pub const fn view(&self) -> Pitch<&T> {
        Pitch { value: self.get() }
    }
    /// returns a new instance containing a mutable reference to the inner value
    pub const fn view_mut(&mut self) -> Pitch<&mut T> {
        Pitch {
            value: self.get_mut(),
        }
    }
}

impl<T> PartialEq<T> for Pitch<T>
where
    T: RawPitch + PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        self.get() == other
    }
}

impl<T> PartialOrd<T> for Pitch<T>
where
    T: RawPitch + PartialOrd,
{
    fn partial_cmp(&self, other: &T) -> Option<core::cmp::Ordering> {
        self.get().partial_cmp(other)
    }
}

#[cfg(feature = "rand")]
impl<T> Pitch<T>
where
    T: RawPitch,
    rand_distr::StandardUniform: rand_distr::Distribution<T>,
{
    pub fn random() -> Self {
        Pitch::new(rand::random())
    }
    pub fn random_in<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        Pitch::new(rng.random())
    }
}

#[cfg(feature = "rand")]
impl<T> rand_distr::Distribution<Pitch<T>> for rand_distr::StandardUniform
where
    T: RawPitch,
    rand_distr::StandardUniform: rand_distr::Distribution<T>,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Pitch<T> {
        Pitch::new(rng.random())
    }
}

impl<T> AsRef<T> for Pitch<T>
where
    T: RawPitch,
{
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<T> AsMut<T> for Pitch<T>
where
    T: RawPitch,
{
    fn as_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> core::borrow::Borrow<T> for Pitch<T>
where
    T: RawPitch,
{
    fn borrow(&self) -> &T {
        self.get()
    }
}

impl<T> core::borrow::BorrowMut<T> for Pitch<T>
where
    T: RawPitch,
{
    fn borrow_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> core::ops::Deref for Pitch<T>
where
    T: RawPitch,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> core::ops::DerefMut for Pitch<T>
where
    T: RawPitch,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<T> core::ops::Neg for Pitch<T>
where
    T: RawPitch + core::ops::Neg,
{
    type Output = Pitch<<T as core::ops::Neg>::Output>;

    fn neg(self) -> Self::Output {
        self.map(|value| -value)
    }
}

impl<T> core::ops::Not for Pitch<T>
where
    T: RawPitch + core::ops::Not,
{
    type Output = Pitch<<T as core::ops::Not>::Output>;

    fn not(self) -> Self::Output {
        self.map(|value| !value)
    }
}

impl<T> num_traits::One for Pitch<T>
where
    T: RawPitch + num_traits::One,
{
    fn one() -> Self {
        Pitch::new(T::one())
    }
}

impl<T> num_traits::Zero for Pitch<T>
where
    T: RawPitch + num_traits::Zero,
{
    fn zero() -> Self {
        Pitch::new(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.get().is_zero()
    }
}

impl<T> num_traits::Num for Pitch<T>
where
    T: RawPitch + num_traits::Num,
{
    type FromStrRadixErr = T::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(str, radix).map(Pitch::new)
    }
}

macro_rules! impl_fmt {
    (@impl $s:ident<$T:ident>($fmt:ident) $(where $($rest:tt)*)?) => {
        impl<$T> ::core::fmt::$fmt for $s<$T>
        where
            $T: ::core::fmt::$fmt,
            $($($rest)*)?,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::$fmt::fmt(self.get(), f)
            }
        }
    };
    ($s:ident<$T:ident>($($fmt:ident),* $(,)?)) => {
        $(
            impl_fmt!(@impl $s<$T>($fmt) where $T: RawPitch);
        )*
    };
}

macro_rules! impl_assign_op {
    (@impl $trait:ident::$method:ident) => {
        impl<A, B> ::core::ops::$trait<Pitch<B>> for Pitch<A>
        where
            A: RawPitch + ::core::ops::$trait<B>,
            B: RawPitch,
        {
            fn $method(&mut self, rhs: Pitch<B>) {
                ::core::ops::$trait::$method(self.get_mut(), rhs.value())
            }
        }

        impl<'a, A, B> ::core::ops::$trait<&'a Pitch<B>> for Pitch<A>
        where
            A: RawPitch + ::core::ops::$trait<&'a B>,
            B: RawPitch,
        {
            fn $method(&mut self, rhs: &'a Pitch<B>) {
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
        impl<A, B, C> ::core::ops::$trait<Pitch<B>> for Pitch<A>
        where
            A: RawPitch + ::core::ops::$trait<B, Output = C>,
            B: RawPitch,
            C: RawPitch,
        {
            type Output = Pitch<C>;

            fn $method(self, rhs: Pitch<B>) -> Self::Output {
                Pitch::new(::core::ops::$trait::$method(self.value(), rhs.value()))
            }
        }

        impl<'a, A, B, C> ::core::ops::$trait<&'a Pitch<B>> for Pitch<A>
        where
            A: ::core::ops::$trait<&'a B, Output = C>,
            A: RawPitch,
            B: RawPitch,
            C: RawPitch,
        {
            type Output = Pitch<C>;

            fn $method(self, rhs: &'a Pitch<B>) -> Self::Output {
                Pitch::new(::core::ops::$trait::$method(self.value(), rhs.get()))
            }
        }

        impl<'a, A, B, C> ::core::ops::$trait<&'a Pitch<B>> for &'a Pitch<A>
        where
            &'a A: ::core::ops::$trait<&'a B, Output = C>,
            A: RawPitch,
            B: RawPitch,
            C: RawPitch,
        {
            type Output = Pitch<C>;

            fn $method(self, rhs: &'a Pitch<B>) -> Self::Output {
                Pitch::new(::core::ops::$trait::$method(self.get(), rhs.get()))
            }
        }

        impl<'a, A, B, C> ::core::ops::$trait<Pitch<B>> for &'a Pitch<A>
        where
            &'a A: ::core::ops::$trait<B, Output = C>,
            A: RawPitch,
            B: RawPitch,
            C: RawPitch,
        {
            type Output = Pitch<C>;

            fn $method(self, rhs: Pitch<B>) -> Self::Output {
                Pitch::new(::core::ops::$trait::$method(self.get(), rhs.value()))
            }
        }
    };

    ($($trait:ident::$method:ident),* $(,)?) => {

        $(
            impl_bin_op!(@impl $trait::$method);
        )*

    };
}

impl_fmt! {
    Pitch<T>(Binary, Debug, Display, LowerExp, LowerHex, Octal, UpperExp, UpperHex)
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
