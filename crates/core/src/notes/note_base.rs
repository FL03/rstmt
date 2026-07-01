/*
    Appellation: aspn <module>
    Contrib: @FL03
*/

use crate::octave::Octave;
use crate::pitch::{self, Accidental, PitchClass, RawPitchClass};

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
    K: Accidental,
{
    pub(crate) class: PitchClass<P, K>,
    pub(crate) octave: Octave,
}
