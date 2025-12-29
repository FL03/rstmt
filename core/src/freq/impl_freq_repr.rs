/*
    appellation: impl_freq_repr <module>
    authors: @FL03
*/
use crate::freq::{Frequency, RawFrequency};
#[cfg(feature = "complex")]
use num_complex::{Complex, ComplexFloat};

impl<T> Frequency<&T>
where
    T: RawFrequency,
{
    #[inline]
    /// returns a new instance of the [`Frequency`] with a cloned instance of the current value.
    pub fn cloned(&self) -> Frequency<T>
    where
        T: Clone,
    {
        Frequency(self.0.clone())
    }

    /// returns a new instance of the [`Frequency`] with a copied instance of the current value.
    pub const fn copied(&self) -> Frequency<T>
    where
        T: Copy,
    {
        Frequency(*self.0)
    }
}

impl<T> Frequency<&mut T>
where
    T: RawFrequency,
{
    #[inline]
    /// returns a new instance of the [`Frequency`] with a cloned instance of the current value.
    pub fn cloned(&self) -> Frequency<T>
    where
        T: Clone,
    {
        Frequency(self.0.clone())
    }

    /// returns a new instance of the [`Frequency`] with a copied instance of the current value.
    pub const fn copied(&self) -> Frequency<T>
    where
        T: Copy,
    {
        Frequency(*self.0)
    }
}

#[cfg(feature = "complex")]
impl<T> Frequency<Complex<T>>
where
    T: RawFrequency,
    Complex<T>: ComplexFloat<Real = T> + RawFrequency,
{
    /// returns the complex conjugate of the frequency value
    pub fn conj(&self) -> Frequency<Complex<T>>
    where
        T: Copy,
    {
        self.map(|v| v.conj())
    }
}
