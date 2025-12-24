/*
    Appellation: impl_triad_base <module>
    Created At: 2025.12.20:10:56:30
    Contrib: @FL03
*/
use crate::triad::TriadBase;

use crate::traits::{RawTriad, RawTriadMut, TriadCls};
use crate::types::LPR;
use num_traits::{Float, FromPrimitive, Num, ToPrimitive};
use rstmt_core::{Octave, PitchMod};

impl<T, S, K> TriadBase<S, K, T>
where
    K: TriadCls,
    S: RawTriad<Elem = T>,
{
    /// Returns a new instance of the [`TriadBase`] with the given chord and kind.
    pub const fn new(chord: S, class: K) -> Self {
        Self {
            chord,
            class,
            octave: Octave(0),
        }
    }
    /// Create a new triad from a root pitch and class
    pub fn from_root_with_class(root: T, class: K) -> Self
    where
        T: Copy + FromPrimitive + PitchMod<Output = T> + core::ops::Add<Output = T>,
    {
        // generate the chord factors from the root and class
        let chord = [
            root,
            (root + T::from_usize(class.root()).unwrap()).pmod(),
            (root + T::from_usize(class.fifth()).unwrap()).pmod(),
        ];
        Self {
            class,
            chord: S::from_arr(chord),
            octave: rstmt_core::Octave(0),
        }
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
    pub const fn octave(&self) -> Octave {
        self.octave
    }
    pub const fn octave_mut(&mut self) -> &mut Octave {
        &mut self.octave
    }
    /// returns a reference to the root note of the triad.
    pub fn root(&self) -> &T
    where
        S: RawTriad,
    {
        self.chord().root()
    }
    /// returns a mutable reference to the root note of the triad.
    pub fn root_mut(&mut self) -> &mut T
    where
        S: RawTriadMut,
    {
        self.chord_mut().root_mut()
    }
    /// returns a reference to the third note of the triad.
    pub fn third(&self) -> &T
    where
        S: RawTriad,
    {
        self.chord().third()
    }
    /// returns a mutable reference to the third note of the triad.
    pub fn third_mut(&mut self) -> &mut T
    where
        S: RawTriadMut,
    {
        self.chord_mut().third_mut()
    }
    /// returns a reference to the fifth note of the triad.
    pub fn fifth(&self) -> &T
    where
        S: RawTriad,
    {
        self.chord().fifth()
    }
    /// returns a mutable reference to the fifth note of the triad.
    pub fn fifth_mut(&mut self) -> &mut T
    where
        S: RawTriadMut,
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
    #[inline]
    /// consumes the current instance to create another with the given chord
    pub fn with_chord<S2>(self, chord: S2) -> TriadBase<S2, K>
    where
        S2: RawTriad<Elem = T>,
    {
        TriadBase {
            chord,
            class: self.class,
            octave: self.octave,
        }
    }
    #[inline]
    /// consumes the current instance to create another with the given class
    pub fn with_class<K2>(self, class: K2) -> TriadBase<S, K2>
    where
        K2: TriadCls,
    {
        TriadBase {
            chord: self.chord,
            class,
            octave: self.octave,
        }
    }
    #[inline]
    pub fn with_octave(self, octave: Octave) -> Self {
        Self { octave, ..self }
    }
    /// consumes the triad and returns the chord and class
    pub fn into_parts(self) -> (S, K) {
        (self.chord, self.class)
    }
    /// returns true if the triad is classified as an augmented triad.
    pub fn is_augmented(&self) -> bool {
        self.class().is_augmented()
    }
    /// returns true if the triad is classified as a diminished triad.
    pub fn is_diminished(&self) -> bool {
        self.class().is_diminished()
    }
    /// returns true if the triad is classified as a major triad.
    pub fn is_major(&self) -> bool {
        self.class().is_major()
    }
    /// returns true if the triad is classified as a minor triad.
    pub fn is_minor(&self) -> bool {
        self.class().is_minor()
    }
    /// computes the centroid of the triad
    pub fn centroid<U>(&self) -> Option<[U; 2]>
    where
        U: Float + FromPrimitive + ToPrimitive + core::iter::Sum<T>,
        S: Clone + IntoIterator<Item = T>,
    {
        let y = U::from_isize(*self.octave)?;
        let x = self.chord().clone().into_iter().sum::<U>() / U::from_u8(3)?;
        Some([x, y])
    }

    /// returns true if the triad contains the given note
    pub fn contains<Q>(&self, note: &Q) -> bool
    where
        for<'a> &'a S: IntoIterator<Item = &'a T>,
        T: PartialEq,
        Q: core::borrow::Borrow<T>,
    {
        self.chord().into_iter().any(|n| n == note.borrow())
    }
    /// returns a collection containing any tones common to both triads
    pub fn common_tones(&self, other: &Self) -> Vec<T>
    where
        T: Clone + PartialEq,
        for<'a> &'a S: IntoIterator<Item = &'a T>,
    {
        self.chord()
            .into_iter()
            .cloned()
            .filter(|n| other.contains(n))
            .collect::<Vec<T>>()
    }
}

impl<S, T, K> TriadBase<S, K, T>
where
    S: RawTriad<Elem = T>,
    K: TriadCls,
    T: Copy + PitchMod<Output = T> + Num + FromPrimitive + ToPrimitive,
{
    pub fn transform<Q>(&self, step: LPR) -> crate::Result<TriadBase<[T; 3], Q, T>>
    where
        K: TriadCls<Rel = Q>,
        Q: TriadCls<Rel = K>,
    {
        step.try_apply(self)
    }
    /// apply the leading transformation to the triad
    pub fn leading<Q>(&self) -> crate::Result<TriadBase<[T; 3], Q, T>>
    where
        K: TriadCls<Rel = Q>,
        Q: TriadCls<Rel = K>,
    {
        self.transform(LPR::Leading)
    }
    /// apply the parallel transformation to the triad
    pub fn parallel<Q>(&self) -> crate::Result<TriadBase<[T; 3], Q, T>>
    where
        K: TriadCls<Rel = Q>,
        Q: TriadCls<Rel = K>,
    {
        self.transform(LPR::Parallel)
    }
    /// apply the relative transformation to the triad
    pub fn relative<Q>(&self) -> crate::Result<TriadBase<[T; 3], Q, T>>
    where
        K: TriadCls<Rel = Q>,
        Q: TriadCls<Rel = K>,
    {
        self.transform(LPR::Relative)
    }
}
