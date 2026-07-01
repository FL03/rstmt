/*
    appellation: impl_freq_rand <module>
    authors: @FL03
*/
#![cfg(feature = "rand")]
use crate::freq::Frequency;
use rand::{Rng, RngExt};
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
    /// generates a random frequency.
    pub fn random() -> Self {
        Frequency::random_with(&mut rand::rng())
    }
    /// generates a random frequency using the provided random number generator.
    pub fn random_with<R>(rng: &mut R) -> Self
    where
        R: ?Sized + Rng,
    {
        Frequency(rng.random())
    }
}

impl<T> Distribution<Frequency<T>> for StandardUniform
where
    StandardUniform: Distribution<T>,
{
    fn sample<R>(&self, rng: &mut R) -> Frequency<T>
    where
        R: ?Sized + Rng,
    {
        Frequency(rng.random())
    }
}
