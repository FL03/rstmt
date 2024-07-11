/*
    Appellation: factors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use strum::IntoEnumIterator;

/// A [chord factor](ChordFactor) describes the position of a note within a [triad](crate::Triad).
/// The `root` factor is the first note of the triad, the `third` factor is the
/// second note of the triad, and the `fifth` factor is the third note of the triad.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumDiscriminants,
    strum::EnumIs,
    strum::VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase"),
    strum_discriminants(
        derive(serde::Deserialize, serde::Serialize),
        serde(rename_all = "lowercase")
    )
)]
#[strum_discriminants(
    name(Factors),
    derive(
        Hash,
        Ord,
        PartialOrd,
        strum::AsRefStr,
        strum::Display,
        strum::EnumCount,
        strum::EnumIter,
        strum::EnumString,
        strum::VariantNames
    )
)]
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub enum ChordFactor<T = ()> {
    #[strum(serialize = "r", serialize = "root")]
    Root(T) = 0,
    #[strum(serialize = "t", serialize = "third")]
    Third(T) = 1,
    #[strum(serialize = "f", serialize = "fifth")]
    Fifth(T) = 2,
}

impl<T> ChordFactor<T> {
    pub fn fifth(data: T) -> Self {
        Self::Fifth(data)
    }

    pub fn root(data: T) -> Self {
        Self::Root(data)
    }

    pub fn third(data: T) -> Self {
        Self::Third(data)
    }
}



mod impl_factors {
    use super::*;
    
    impl Factors {
        pub fn root() -> Self {
            Self::Root
        }

        pub fn third() -> Self {
            Self::Third
        }

        pub fn fifth() -> Self {
            Self::Fifth
        }

        pub fn factors() -> [Self; 3] {
            use Factors::*;
            [Root, Third, Fifth]
        }
    
        pub fn others(&self) -> Vec<Self> {
            Self::iter().filter(|x| x != self).collect()
        }
    }
    
    impl Default for Factors {
        fn default() -> Self {
            Factors::Root
        }
    }
    
    impl From<usize> for Factors {
        fn from(x: usize) -> Self {
            match x % 3 {
                0 => Factors::Root,
                1 => Factors::Third,
                _ => Factors::Fifth,
            }
        }
    }
    
    impl From<Factors> for usize {
        fn from(x: Factors) -> Self {
            x as usize
        }
    }
    
    unsafe impl petgraph::graph::IndexType for Factors {
        fn new(x: usize) -> Self {
            Self::from(x)
        }
    
        fn index(&self) -> usize {
            *self as usize
        }
    
        fn max() -> Self {
            Self::Fifth
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chord_factors() {

        // let triad =
    }

    #[test]
    fn chord_factors_iter() {
        use Factors::*;

        let factors = Factors::factors();
        assert_eq!(factors.len(), 3);
        assert_eq!(factors[0], Root);
        assert_eq!(factors[1], Third);
        assert_eq!(factors[2], Fifth);
    }
}
