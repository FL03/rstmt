/*
    Appellation: absmod <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::pitch::PitchTy;
use num::traits::Signed;

pub trait AbsMod<Rhs = Self> {
    type Output;

    fn absmod(&self, rhs: Rhs) -> Self::Output;
}

pub trait PitchMod {
    const MOD: PitchTy = crate::MODULUS;
    type Output;

    fn pitchmod(&self) -> Self::Output;
}

impl<S> PitchMod for S
where
    S: AbsMod<PitchTy>,
{
    type Output = <S as AbsMod<PitchTy>>::Output;

    fn pitchmod(&self) -> Self::Output {
        self.absmod(Self::MOD)
    }
}

/*
 ************* Implementations *************
*/

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
