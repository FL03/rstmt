/*
    Appellation: scale <module>
    Created At: 2025.12.29:16:46:22
    Contrib: @FL03
*/
use crate::Frequency;

/// The [`Scale`] implementation uses the defined _root_ or _anchor_ frequency as the basis for
/// classfifications and other related computations.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Scale<T = f32> {
    pub root: Frequency<T>,
}

#[cfg(test)]
mod tests {
    use super::Scale;
    use crate::freq::Frequency;

    const A4: Frequency<f64> = Frequency(440.0);
    const C4: Frequency<f64> = Frequency(261.6255653005986);

    #[test]
    fn test_freq_converter() {
        let n: isize = -9; // C4;
        let scale = Scale::from_freq(A4); // A4

        assert! { (scale.get_freq_of_class(n) - C4).abs() < 1e-5 }
        assert_eq! { scale.classify(C4), Some(n) }
    }
}
