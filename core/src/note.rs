/*
    Appellation: aspn <module>
    Contrib: @FL03
*/
mod impl_aspn;
mod impl_aspn_ext;
mod impl_note_base;
mod impl_note_ext;
mod impl_note_repr;

use crate::octave::Octave;
use crate::pitch::{self, PitchClass, RawAccidental, RawPitchClass};

/// The [`AsAspn`] trait is used to convert a reference into a [`Aspn`]
pub trait AsAspn {
    fn as_aspn(&self) -> Aspn;
}
/// A trait for converting a type into a [`Aspn`]
pub trait IntoAspn {
    fn into_aspn(self) -> Aspn;
}

/// An american scientific pitch notation ([`Aspn`]) representation of a musical note; this
/// standard is used to represent notes in a way that is consistent with the
/// American scientific pitch notation system, which uses a combination of a pitch class
/// (represented as an integer) and an octave (represented as an [`Octave`]) to uniquely
/// identify a musical note. The pitch class is the note's position in the chromatic scale,
/// while the octave indicates the note's position in the musical range.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields, default, rename_all = "snake_case")
)]
#[repr(C)]
pub struct Aspn {
    pub(crate) class: usize,
    pub(crate) octave: Octave,
}

/// The [`NoteBase`] is a generic representation of a musical note
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
#[repr(C)]
pub struct NoteBase<P = pitch::CNote, K = <P as RawPitchClass>::Tag>
where
    P: RawPitchClass<Tag = K>,
    K: RawAccidental,
{
    pub(crate) class: PitchClass<P, K>,
    pub(crate) octave: Octave,
}

/*
 ************* Implementations *************
*/
impl<T> AsAspn for T
where
    T: Clone + IntoAspn,
{
    fn as_aspn(&self) -> Aspn {
        self.clone().into_aspn()
    }
}

impl<T> IntoAspn for T
where
    T: Into<Aspn>,
{
    fn into_aspn(self) -> Aspn {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use super::NoteBase;
    use crate::octave::Octave;
    use crate::pitch::{C, CNote};

    #[test]
    fn test_note_base_parse() {
        assert_eq! { NoteBase::<CNote>::from_octave(Octave(4)), "C.4" }
        assert_eq! { "C.4".parse::<NoteBase<_, _>>().unwrap(), NoteBase::new(C::new(), Octave(4)) }
    }
}
