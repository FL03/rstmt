/*
    Appellation: harmonic <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(unused)]
use num::traits::{Num, NumOps};

pub trait Frequency<T> {
    fn freq(self) -> T;

    fn period(self) -> T;
}

pub trait Timbre {}

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
