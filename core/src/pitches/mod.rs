/*
    Appellation: pitches <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{kinds::*, pitch::Pitch, utils::*};

pub(crate) mod kinds;
pub(crate) mod pitch;

/// A type alias for an integer representing a particular pitch of a note
pub type PitchTy = i8;

pub trait PitchClass {
    fn pitch(&self) -> PitchTy;
}

pub trait Pitched {
    /// Classify the pitch into a pitch class
    fn class(&self) -> Pitches {
        self.pitch().into()
    }
    /// Find the modular index of the given pitch
    fn pitch(&self) -> PitchTy;
}

pub(crate) mod prelude {
    pub use super::kinds::*;
    pub use super::{PitchClass, PitchTy};
}

pub(crate) mod utils {
    use num::traits::Signed;

    pub trait AbsMod<Rhs = Self> {
        type Output;

        fn absmod(&self, rhs: Rhs) -> Self::Output;
    }

    impl<A, B, C> AbsMod<B> for A
    where
        A: Copy + core::ops::Rem<B, Output = C> + core::ops::Add<C, Output = C>,
        B: Copy,
        C: core::ops::Add<B, Output = C> + core::ops::Rem<B, Output = C> + Signed,
    {
        type Output = C;

        fn absmod(&self, rhs: B) -> Self::Output {
            (((*self % rhs) + rhs) % rhs).abs()
        }
    }
    /// [absmod] is short for the absolute value of a modular number;
    pub fn absmod(a: i64, m: i64) -> i64 {
        (((a % m) + m) % m).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_absmod() {
        assert_eq!(absmod(5, 12), 5.absmod(12));
        assert_eq!(absmod(-5, 12), (-5).absmod(12));
        assert_eq!(absmod(0, 12), 0.absmod(12));
        assert_eq!(absmod(12, 12), 24.absmod(12));
        assert_eq!(absmod(13, 12), 1.absmod(12));
        assert_eq!(absmod(-13, 12), 11.absmod(12));
    }
}
