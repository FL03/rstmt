/*
    Appellation: harmonic <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Frequency<T> {
    fn freq(self) -> T;

    fn period(self) -> T;
}

pub trait Timbre {}
