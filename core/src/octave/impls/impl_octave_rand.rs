/*
    appellation: impl_octave_rand <module>
    authors: @FL03
*/
use crate::octave::Octave;
use rand::RngCore;
use rand_distr::uniform::{SampleRange, SampleUniform};
use rand_distr::{Distribution, StandardNormal, StandardUniform};

impl<T> Octave<T> {
    /// generate a random octave
    pub fn random() -> Self
    where
        StandardUniform: Distribution<T>,
    {
        Octave(rand::random())
    }
    /// generates a random octave using the provided random number generator.
    pub fn random_in<R>(rng: &mut R) -> Self
    where
        R: ?Sized + RngCore,
        StandardUniform: Distribution<T>,
    {
        use rand::Rng;
        Octave(rng.random())
    }
    /// generates a random octave using the provided random number generator.
    pub fn random_with<R, Distr>(rng: &mut R, distr: Distr) -> Self
    where
        R: ?Sized + RngCore,
        Distr: Distribution<T>,
    {
        use rand::Rng;
        Octave(rng.sample(distr))
    }
    /// generates a random octave within the specified range.
    pub fn random_range<R>(range: R) -> Self
    where
        R: SampleRange<T>,
        T: SampleUniform,
    {
        Self(rand::random_range(range))
    }
}

impl<T> Distribution<Octave<T>> for StandardUniform
where
    StandardUniform: Distribution<T>,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Octave<T> {
        Octave::random_with(rng, StandardUniform)
    }
}

impl<T> Distribution<Octave<T>> for StandardNormal
where
    StandardNormal: Distribution<T>,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Octave<T> {
        Octave::random_with(rng, StandardNormal)
    }
}
