/*
    appellation: impl_freq_rand <module>
    authors: @FL03
*/
use crate::freq::{Frequency, RawFrequency};
use rand_distr::uniform::{SampleRange, SampleUniform};
use rand_distr::{Distribution, StandardUniform};

impl<T> Frequency<T> {
    /// generates a new random frequency within the specified range.
    pub fn random_range<R>(range: R) -> Self
    where
        R: SampleRange<T>,
        T: SampleUniform,
    {
        Frequency(rand::random_range(range))
    }
}

impl<T> Frequency<T>
where
    StandardUniform: Distribution<T>,
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
