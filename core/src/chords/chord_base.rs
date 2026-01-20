/*
    Appellation: chord_base <module>
    Created At: 2026.01.20:08:24:06
    Contrib: @FL03
*/
use super::RawChord;

pub type ChordSlice<T> = ChordBase<[T], T>;

pub type ChordArray<T, const N: usize> = ChordBase<[T; N], T>;

pub type ChordSliceRef<'a, T> = ChordBase<&'a [T], T>;

pub type ChordSliceMut<'a, T> = ChordBase<&'a mut [T], T>;

#[cfg(feature = "alloc")]
/// a type alias for a [`ChordBase`] that leverages a [`Vec`](alloc::vec::Vec) as its underlying
/// representation.
pub type Chord<T> = ChordBase<alloc::vec::Vec<T>, T>;

/// The [`ChordBase`] implementation is designed to be a generic container allowing
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields, rename_all = "snake_case")
)]
#[repr(C)]
pub struct ChordBase<R, T = <R as RawChord>::Elem>
where
    R: ?Sized + RawChord<Elem = T>,
{
    pub(crate) repr: R,
}
