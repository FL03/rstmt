/*
    Appellation: impl_pclass_ops <module>
    Created At: 2025.12.23:14:33:56
    Contrib: @FL03
*/
use crate::pitch::pitch_class::PitchClass;
use crate::pitch::traits::{RawAccidental, RawPitchClass};
use rstmt_traits::PitchMod;

/// Add two pitch-classes producing a wrapped semitone count in the tonal space.
///
/// Note: returning a compile-time `PitchClass` subtype from an operator requires a
/// type-level index; that's not possible for arbitrary runtime sums. We therefore
/// return an `i32` semitone value (0..11) using `PitchMod::pmod`. Callers can
/// convert that numeric result into any runtime/compile-time representation they
/// prefer.
impl<T1, A1, T2, A2> core::ops::Add<PitchClass<T2, A2>> for PitchClass<T1, A1>
where
    T1: RawPitchClass<Tag = A1>,
    A1: RawAccidental,
    T2: RawPitchClass<Tag = A2>,
    A2: RawAccidental,
{
    type Output = isize;

    fn add(self, rhs: PitchClass<T2, A2>) -> Self::Output {
        (self.get().index() + rhs.get().index()).pmod()
    }
}

impl<T1, A1, T2, A2> core::ops::Sub<PitchClass<T2, A2>> for PitchClass<T1, A1>
where
    T1: RawPitchClass<Tag = A1>,
    A1: RawAccidental,
    T2: RawPitchClass<Tag = A2>,
    A2: RawAccidental,
{
    /// Subtract two pitch-classes producing a wrapped signed semitone interval.
    type Output = isize;

    fn sub(self, rhs: PitchClass<T2, A2>) -> Self::Output {
        (self.get().index() - rhs.get().index()).pmod()
    }
}
