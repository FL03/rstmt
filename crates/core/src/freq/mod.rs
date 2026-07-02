/*
    Appellation: freq <module>
    Created At: 2026.01.20:08:34:12
    Contrib: @FL03
*/
#[doc(inline)]
pub use self::{frequency::*, traits::*, utils::*};

mod frequency;

mod impls {
    mod impl_freq;
    mod impl_freq_ext;
    mod impl_freq_rand;
    mod impl_freq_repr;
}

mod traits {
    #[doc(inline)]
    pub use self::{convert::*, raw_frequency::*};

    mod convert;
    mod raw_frequency;
}

mod utils {
    #[doc(inline)]
    pub use self::frequency::*;

    mod frequency;
}
// prelude (local)
pub(crate) mod prelude {
    pub use super::frequency::*;
    pub use super::traits::*;
    pub use super::utils::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    const A4: f64 = 440.0;
    const C4: f64 = 261.6255653005986;

    #[test]
    fn test_freq_classification() {
        let f = Frequency::new(C4);
        assert_eq! { f.classify_by(A4), -9 }
    }
}
