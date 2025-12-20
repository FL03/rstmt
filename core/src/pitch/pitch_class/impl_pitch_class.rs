/*
    Appellation: impl_pitch_class <module>
    Created At: 2025.12.20:08:51:41
    Contrib: @FL03
*/
use super::PitchClass;
use crate::pitch::{PitchCls, PitchType};

impl<N, T> core::fmt::Debug for PitchClass<N, T>
where
    N: PitchCls + core::fmt::Debug,
    T: PitchType,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.class)
    }
}

impl<N, T> core::fmt::Display for PitchClass<N, T>
where
    N: PitchCls + core::fmt::Display,
    T: PitchType,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.class)
    }
}

impl<N, T> PitchCls for PitchClass<N, T>
where
    N: PitchCls,
    T: PitchType,
{
    const IDX: usize = N::IDX;

    seal! {}

    fn new() -> Self {
        Self {
            class: N::new(),
            _marker: core::marker::PhantomData,
        }
    }

    fn index(&self) -> usize {
        self.class.index()
    }
}

unsafe impl<N, T> Send for PitchClass<N, T>
where
    N: PitchCls,
    T: PitchType,
{
}

unsafe impl<N, T> Sync for PitchClass<N, T>
where
    N: PitchCls,
    T: PitchType,
{
}
