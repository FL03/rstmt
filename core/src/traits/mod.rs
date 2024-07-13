/*
    Appellation: traits <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::prelude::*;

pub mod absmod;
pub mod distance;

pub(crate) mod prelude {
    pub use super::absmod::*;
    pub use super::distance::*;
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_absmod() {
        use super::absmod::{AbsMod, PitchMod};
        let pairs = [(5, 12), (-5, 12), (0, 12), (12, 12), (13, 12), (-13, 12)];
        for (i, j) in pairs.iter() {
            assert_eq!(i.absmod(*j), i.pitchmod());
        }
    }
}
