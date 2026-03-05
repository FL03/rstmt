/*
    Appellation: impl_pitch_rand <module>
    Created At: 2025.12.20:09:07:03
    Contrib: @FL03
*/
#![cfg(feature = "rand")]
use crate::pitch::Pitch;

use rand::{Rng, RngExt};
use rand_distr::{Distribution, StandardUniform};

impl<T> Pitch<T>
where
    StandardUniform: Distribution<T>,
{
    /// initialize a new pitch with a random value
    pub fn random() -> Self {
        Pitch::rand_from_rng(&mut rand::rng())
    }
    /// initialize a new pitch using the given random number generator
    pub fn rand_from_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
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
