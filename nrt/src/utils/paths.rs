/*
    Appellation: paths <module>
    Contrib: @FL03
*/
use crate::{LPR, Triad};

/// Determine if there's a single transformation between two triads
pub fn get_transformation(triad1: &Triad, triad2: &Triad) -> Option<LPR> {
    use strum::IntoEnumIterator;
    for transform in LPR::iter() {
        let result = triad1.transform(transform);
        if result == *triad2 {
            return Some(transform);
        }
    }

    None
}
