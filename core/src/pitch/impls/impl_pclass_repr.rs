/*
    Appellation: impl_pclass_repr <module>
    Created At: 2025.12.31:16:01:04
    Contrib: @FL03
*/
use crate::pitch::pitch_class::PitchClass;

use crate::pitch::{Flat, Natural, RawPitchClass, Sharp};

impl<N> PitchClass<N>
where
    N: RawPitchClass,
{
    /// initialize a new, _natural_ pitch class
    pub const fn natural(class: N) -> PitchClass<N, Natural>
    where
        N: RawPitchClass<Tag = Natural>,
    {
        PitchClass {
            class,
            kind: Natural,
        }
    }
    /// initialize a _sharp_ instance of the given class
    pub const fn sharp(class: N) -> PitchClass<N, Sharp>
    where
        N: RawPitchClass<Tag = Sharp>,
    {
        PitchClass { class, kind: Sharp }
    }
    /// initialize a _flat_ instance of the given class
    pub const fn flat(class: N) -> PitchClass<N, Flat>
    where
        N: RawPitchClass<Tag = Flat>,
    {
        PitchClass { class, kind: Flat }
    }
}

impl<N> PitchClass<N, Natural> where N: RawPitchClass<Tag = Natural> {}

impl<N> PitchClass<N, Sharp> where N: RawPitchClass<Tag = Sharp> {}

impl<N> PitchClass<N, Flat> where N: RawPitchClass<Tag = Flat> {}
