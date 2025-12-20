/*
    Appellation: note <module>
    Contrib: @FL03
*/
mod impl_note_base;

use crate::octave::Octave;
use crate::pitch::{self, Pitch, PitchCls};

/// The [`NoteBase`] is a generic representation of a musical note
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields, rename_all = "snake_case")
)]
#[repr(C)]
pub struct NoteBase<P, K = pitch::C>
where
    K: PitchCls,
{
    pub(crate) class: K,
    pub(crate) octave: Octave,
    pub(crate) pitch: Pitch<P>,
}
