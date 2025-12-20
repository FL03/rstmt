/*
    Appellation: impl_triad_base <module>
    Created At: 2025.12.20:10:56:30
    Contrib: @FL03
*/
use crate::triad::TriadBase;

use crate::traits::{RawTriadStore, RawTriadStoreMut, TriadKind};

impl<T, S, K> TriadBase<S, K, T>
where
    K: TriadKind,
    S: RawTriadStore<Elem = T>,
{
    /// Returns a new instance of the [`TriadBase`] with the given chord and kind.
    pub const fn new(chord: S, class: K) -> Self {
        Self { chord, class }
    }
    /// returns an immutable reference to the chord.
    pub const fn chord(&self) -> &S {
        &self.chord
    }
    /// returns a mutable reference to the chord.
    pub const fn chord_mut(&mut self) -> &mut S {
        &mut self.chord
    }
    /// returns a copy of the class of the triad.
    pub const fn class(&self) -> K {
        self.class
    }
    /// returns a mutable reference to the class of the triad.
    pub const fn class_mut(&mut self) -> &mut K {
        &mut self.class
    }
    /// returns a reference to the root note of the triad.
    pub fn root(&self) -> &T
    where
        S: RawTriadStore,
    {
        self.chord().root()
    }
    /// returns a mutable reference to the root note of the triad.
    pub fn root_mut(&mut self) -> &mut T
    where
        S: RawTriadStoreMut,
    {
        self.chord_mut().root_mut()
    }
    /// returns a reference to the third note of the triad.
    pub fn third(&self) -> &T
    where
        S: RawTriadStore,
    {
        self.chord().third()
    }
    /// returns a mutable reference to the third note of the triad.
    pub fn third_mut(&mut self) -> &mut T
    where
        S: RawTriadStoreMut,
    {
        self.chord_mut().third_mut()
    }
    /// returns a reference to the fifth note of the triad.
    pub fn fifth(&self) -> &T
    where
        S: RawTriadStore,
    {
        self.chord().fifth()
    }
    /// returns a mutable reference to the fifth note of the triad.
    pub fn fifth_mut(&mut self) -> &mut T
    where
        S: RawTriadStoreMut,
    {
        self.chord_mut().fifth_mut()
    }
    /// update the chord and returns a mutable reference to the triad
    pub fn set_chord(&mut self, chord: S) -> &mut Self {
        self.chord = chord;
        self
    }
    /// update the class and returns a mutable reference to the triad
    pub fn set_class(&mut self, class: K) -> &mut Self {
        self.class = class;
        self
    }
    /// consumes the current instance to create another with the given chord
    pub fn with_chord<S2>(self, chord: S2) -> TriadBase<S2, K>
    where
        S2: RawTriadStore<Elem = T>,
    {
        TriadBase {
            chord,
            class: self.class,
        }
    }
    /// consumes the current instance to create another with the given class
    pub fn with_class<K2>(self, class: K2) -> TriadBase<S, K2>
    where
        K2: TriadKind,
    {
        TriadBase {
            chord: self.chord,
            class,
        }
    }
    /// consumes the triad and returns the chord and class
    pub fn into_parts(self) -> (S, K) {
        (self.chord, self.class)
    }
    /// returns true if the triad is classified as an augmented triad.
    pub fn is_augmented(&self) -> bool {
        use core::any::TypeId;
        TypeId::of::<K>() == TypeId::of::<crate::Augmented>()
    }
    /// returns true if the triad is classified as a diminished triad.
    pub fn is_diminished(&self) -> bool {
        use core::any::TypeId;
        TypeId::of::<K>() == TypeId::of::<crate::Diminished>()
    }
    /// returns true if the triad is classified as a major triad.
    pub fn is_major(&self) -> bool {
        use core::any::TypeId;
        TypeId::of::<K>() == TypeId::of::<crate::Major>()
    }
    /// returns true if the triad is classified as a minor triad.
    pub fn is_minor(&self) -> bool {
        use core::any::TypeId;
        TypeId::of::<K>() == TypeId::of::<crate::Minor>()
    }
}
