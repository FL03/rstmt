/*
    Appellation: impl_pitch_rand <module>
    Created At: 2025.12.20:09:07:03
    Contrib: @FL03
*/
#![cfg(feature = "rand")]
use super::Pitch;
use rand::Rng;
use rand_distr::{Distribution, StandardUniform};

impl<T> Pitch<T>
where
    StandardUniform: Distribution<T>,
{
    pub fn random() -> Self {
        Pitch::random_in(&mut rand::rng())
    }
    /// initialize a new pitch using the given random number generator
    pub fn random_in<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Pitch(rng.random())
    }
}

#[cfg(feature = "rand")]
impl<T> Distribution<Pitch<T>> for StandardUniform
where
    StandardUniform: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Pitch<T> {
        Pitch(rng.sample(StandardUniform))
    }
}
