/*
    appellation: freq <module>
    authors: @FL03
*/
//! The `freq` module defines the [`Frequency`] type and its associated traits and implementations.
#[doc(inline)]
pub use self::{frequency::*, scale::*, traits::*};

mod frequency;
mod scale;

mod impls {
    pub mod impl_freq;
    pub mod impl_freq_ops;
    #[cfg(feature = "rand")]
    pub mod impl_freq_rand;
    pub mod impl_freq_repr;
}

mod traits {
    //! this module provides various traits to support various representations and operations
    //! related to frequencies.
    #[doc(inline)]
    pub use self::{convert::*, frequency::*};

    mod convert;
    mod frequency;
}

pub(crate) mod prelude {
    pub use super::frequency::*;
    pub use super::scale::*;
    pub use super::traits::*;
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
