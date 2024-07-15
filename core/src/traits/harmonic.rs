/*
    Appellation: harmonic <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use num::traits::{Num, NumOps};

pub trait Frequency<T> {
    fn period(self) -> T;
}
pub struct Freq<T = f64> {
    hz: T,
}

impl<T> Freq<T>
where
    T: Num + NumOps,
{
    pub fn period(self) -> T {
        T::one() / self.hz
    }
}

pub trait Tone {}

pub trait Harmonic {
    type Tone: Tone;
}
