/*
    appellation: freq <module>
    authors: @FL03
*/
//! The `freq` module defines the [`Frequency`] type and its associated traits and implementations.
#[doc(inline)]
pub use self::{frequency::*, scale::*};

mod frequency;
mod scale;

pub(crate) mod prelude {
    pub use super::frequency::*;
    pub use super::scale::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freq_convert() {
        let n: isize = -9; // C4;
        let f_exp: f64 = 261.6255653005986;
        let base = ScaleToFrequency::new(440f64); // A4

        let res = base.compute(n).unwrap();

        assert!((res - f_exp).abs() < f64::EPSILON);
        assert_eq!(base.from_scale_degree(res), Some(n));
    }
}
