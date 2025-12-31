/*
    Appellation: impl_pclass_repr <module>
    Created At: 2025.12.31:16:01:04
    Contrib: @FL03
*/
use crate::pitch::pitch_class::PitchClass;

use crate::pitch::{Flat, Natural, PitchClassRepr, Sharp};

impl<N> PitchClass<N>
where
    N: PitchClassRepr,
{
    pub fn natural() -> PitchClass<N, Natural>
    where
        N: PitchClassRepr<Tag = Natural>,
    {
        PitchClass::<N, Natural>::new()
    }

    pub fn sharp(self) -> PitchClass<N, Sharp>
    where
        N: PitchClassRepr<Tag = Sharp>,
    {
        PitchClass::<N, Sharp>::new()
    }

    pub fn flat(self) -> PitchClass<N, Flat>
    where
        N: PitchClassRepr<Tag = Flat>,
    {
        PitchClass::<N, Flat>::new()
    }
}

impl<N> PitchClass<N, Natural> where N: PitchClassRepr<Tag = Natural> {}

impl<N> PitchClass<N, Sharp> where N: PitchClassRepr<Tag = Sharp> {}

impl<N> PitchClass<N, Flat> where N: PitchClassRepr<Tag = Flat> {}
