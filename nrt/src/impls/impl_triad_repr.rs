/*
    Appellation: impl_triad_repr <module>
    Created At: 2025.12.20:11:04:03
    Contrib: @FL03
*/
use crate::triad::TriadBase;

use crate::traits::RawTriadStore;
use rstmt_core::{Augmented, Diminished, Major, Minor};

impl<S> TriadBase<S, Augmented>
where
    S: RawTriadStore,
{
    /// returns a new instance of the [`TriadBase`] with the given chord and kind as an
    /// augmented triad.
    pub const fn augmented(chord: S) -> Self {
        TriadBase::new(chord, Augmented)
    }
}

impl<S> TriadBase<S, Diminished>
where
    S: RawTriadStore,
{
    /// returns a new instance of the [`TriadBase`] with the given chord and kind as a
    /// diminished triad.
    pub const fn diminished(chord: S) -> Self {
        TriadBase::new(chord, Diminished)
    }
}

impl<S> TriadBase<S, Major>
where
    S: RawTriadStore,
{
    /// returns a new instance of the [`TriadBase`] with the given chord and kind as a major
    /// triad.
    pub const fn major(chord: S) -> Self {
        TriadBase::new(chord, Major)
    }
}

impl<S> TriadBase<S, Minor>
where
    S: RawTriadStore,
{
    /// returns a new instance of the [`TriadBase`] with the given chord and kind as a minor
    /// triad.
    pub const fn minor(chord: S) -> Self {
        TriadBase::new(chord, Minor)
    }
}
