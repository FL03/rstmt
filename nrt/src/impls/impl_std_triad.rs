/*
    Appellation: impl_std_triad <module>
    Created At: 2025.12.20:10:40:33
    Contrib: @FL03
*/
use crate::triad::Triad;

use crate::error::TriadError;
use crate::{Factors, LPR, TriadClass};
use rstmt::{Aspn, IntoAspn, Octave, PitchMod};

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use num_traits::{Float, FromPrimitive};

impl Triad {
    pub fn new(notes: [usize; 3], class: TriadClass) -> Self {
        if !class.validate(&notes) {
            panic!("Invalid triad pitches for class {notes:?}");
        }
        Self {
            class,
            notes,
            octave: Octave(4),
        }
    }
    /// Create a new triad from a root pitch and class
    pub fn from_root<N>(root: N, class: TriadClass) -> Self
    where
        N: IntoAspn,
    {
        // all IntoNote implementations should* already compute pmod
        let note = root.into_aspn();
        let root = note.class();
        let [a, .., c] = class.intervals();
        let third = (root + a).pmod();
        let fifth = (root + c).pmod();
        Self {
            class,
            notes: [root, third, fifth],
            octave: note.octave(),
        }
    }
    /// creates a new augmented triad from the given root
    pub fn augmented<N>(root: N) -> Self
    where
        N: IntoAspn,
    {
        Self::from_root(root, TriadClass::Augmented)
    }
    /// creates a new diminished triad from the given root
    pub fn diminished<N>(root: N) -> Self
    where
        N: IntoAspn,
    {
        Self::from_root(root, TriadClass::Diminished)
    }
    /// Create a new major triad from the given root
    pub fn major<N>(root: N) -> Self
    where
        N: IntoAspn,
    {
        Self::from_root(root, TriadClass::Major)
    }
    /// creates a new minor triad from the given root
    pub fn minor<N>(root: N) -> Self
    where
        N: IntoAspn,
    {
        Self::from_root(root, TriadClass::Minor)
    }
    /// returns a copy of the class of the triad
    pub const fn class(&self) -> TriadClass {
        self.class
    }
    /// returns a mutable reference to the class of the triad
    pub const fn class_mut(&mut self) -> &mut TriadClass {
        &mut self.class
    }
    /// returns copy of the notes currently composing the triad
    pub const fn notes(&self) -> &[usize; 3] {
        &self.notes
    }
    /// returns a mutable reference to the notes of the triad
    pub const fn notes_mut(&mut self) -> &mut [usize; 3] {
        &mut self.notes
    }
    /// returns a copy of the octave of the triad
    pub const fn octave(&self) -> Octave {
        self.octave
    }
    /// returns a mutable reference to the octave of the triad
    pub const fn octave_mut(&mut self) -> &mut Octave {
        &mut self.octave
    }
    /// set the octave of the triad
    pub fn set_octave(&mut self, octave: Octave) -> &mut Self {
        self.octave = octave;
        self
    }
    /// consumes the current instance to create another with the given octave
    pub fn with_octave(self, octave: Octave) -> Self {
        Self { octave, ..self }
    }
    /// returns a copy of the root pitch of the triad
    pub fn root(&self) -> usize {
        self[Factors::Root]
    }
    /// returns a mutable reference to the root pitch of the triad
    pub fn root_mut(&mut self) -> &mut usize {
        &mut self[Factors::Root]
    }
    /// returns a copy of the third pitch of the triad
    pub fn third(&self) -> usize {
        self[Factors::Third]
    }
    /// returns a mutable reference to the third pitch of the triad
    pub fn third_mut(&mut self) -> &mut usize {
        &mut self[Factors::Third]
    }
    /// returns a copy of the fifth pitch of the triad
    pub fn fifth(&self) -> usize {
        self[Factors::Fifth]
    }
    /// returns a mutable reference to the fifth pitch of the triad
    pub fn fifth_mut(&mut self) -> &mut usize {
        &mut self[Factors::Fifth]
    }
    /// returns true if the current instance is an augmented triad
    pub fn is_augmented(&self) -> bool {
        self.class().is_augmented()
    }
    /// returns true if the current instance is a diminished triad
    pub fn is_diminished(&self) -> bool {
        self.class().is_diminished()
    }
    /// returns true if the current instance is a major triad
    pub fn is_major(&self) -> bool {
        self.class().is_major()
    }
    /// returns true if the current instance is a minor triad
    pub fn is_minor(&self) -> bool {
        self.class().is_minor()
    }
    /// returns true if the pitches within the triad match its classification
    pub fn is_valid(&self) -> bool {
        self.class().validate(&self.notes())
    }
    /// apply the leading transformation to the triad
    pub fn leading(&self) -> Self {
        self.transform(LPR::Leading)
    }
    /// apply the parallel transformation to the triad
    pub fn parallel(&self) -> Self {
        self.transform(LPR::Parallel)
    }
    /// apply the relative transformation to the triad
    pub fn relative(&self) -> Self {
        self.transform(LPR::Relative)
    }
    // return the barycentric coordinates of the given note w.r.t the current triad
    pub fn barycentric<T>(&self, p: impl IntoAspn) -> [T; 3]
    where
        T: Float + FromPrimitive,
    {
        let note = p.into_aspn();
        let px = T::from_usize(note.class().pmod()).unwrap();
        let py = T::from_isize(*note.octave()).unwrap();
        let y = T::from_isize(*self.octave).unwrap();
        let [v0, v1, v2] = self.notes.map(|n| T::from_usize(n).unwrap());

        let d00 = v0 * v0 + y * y;
        let d01 = v0 * v1 + y * y;
        let d11 = v1 * v1 + y * y;
        let d20 = v2 * px + y * py;
        let d21 = v2 * v1 + y * y;

        let denom = d00 * d11 - d01 * d01;
        let a = (d11 * d20 - d01 * d21) / denom;
        let b = (d00 * d21 - d01 * d20) / denom;
        let c = T::one() - a - b;
        [a, b, c]
    }
    /// computes the centroid of the triad
    pub fn centroid<T>(&self) -> Option<[T; 2]>
    where
        T: Float + FromPrimitive,
    {
        let y = T::from_isize(*self.octave)?;
        let x = T::from_usize(self.notes().iter().sum())? / T::from_usize(self.notes().len())?;
        Some([x, y])
    }
    #[cfg(feature = "alloc")]
    /// returns the number of common tones between two triads
    pub fn common_tones(&self, other: &Self) -> Vec<usize> {
        self.notes()
            .iter()
            .filter(|&&n| other.contains(&n))
            .copied()
            .collect::<Vec<_>>()
    }

    /// check if the triad contains a given pitch class
    pub fn contains<Q>(&self, pitch: &Q) -> bool
    where
        Q: core::borrow::Borrow<usize>,
    {
        self.notes().contains(pitch.borrow())
    }
    /// returns some [`LPR`] transformation, iff they are within a single _step_ of one another
    /// otherwise, returns [`None`](Option::None).
    pub fn is_neighbor(&self, other: &Triad) -> Option<LPR> {
        LPR::iter().find(|&t| {
            let result = self.transform(t);
            result == *other
        })
    }
    #[cfg(feature = "alloc")]
    /// creates an instance of the transformer for the current triad
    pub fn path_finder(&self) -> crate::transform::TriadNavigator<'_> {
        crate::transform::TriadNavigator::new(self)
    }
    /// apply a single [LPR] transformation to a triad
    pub fn transform(&self, transform: LPR) -> Self {
        transform.apply(self)
    }
    /// apply a single transformation to a triad in-place, mutating the current instance
    pub fn transform_inplace(&mut self, transform: LPR) {
        *self = self.transform(transform);
    }
    /// apply a series of transformations to a triad
    pub fn walk<I>(&self, path: I) -> Self
    where
        I: IntoIterator<Item = LPR>,
    {
        path.into_iter()
            .fold(*self, |triad, transform| transform.apply(&triad))
    }
    /// apply a chain of transformations to a triad in-place
    pub fn walk_inplace<I>(&mut self, path: I)
    where
        I: IntoIterator<Item = LPR>,
    {
        *self = self.walk(path);
    }
    /// try to apply a single transformation to a triad
    pub fn try_transform(&self, transform: LPR) -> Result<Self, TriadError> {
        transform.try_apply(self)
    }
}

impl Default for Triad {
    fn default() -> Self {
        Triad::major(Aspn::from_pitch(0))
    }
}

impl core::fmt::Display for Triad {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}({:?})", self.class, self.notes)
    }
}

impl core::convert::AsRef<[usize; 3]> for Triad {
    fn as_ref(&self) -> &[usize; 3] {
        self.notes()
    }
}

impl core::convert::AsMut<[usize; 3]> for Triad {
    fn as_mut(&mut self) -> &mut [usize; 3] {
        self.notes_mut()
    }
}

impl core::ops::Index<usize> for Triad {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        &self.notes[index % 3]
    }
}

impl core::ops::IndexMut<usize> for Triad {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.notes[index % 3]
    }
}

impl core::ops::Index<Factors> for Triad {
    type Output = usize;
    fn index(&self, index: Factors) -> &Self::Output {
        &self.notes[index as usize]
    }
}

impl core::ops::IndexMut<Factors> for Triad {
    fn index_mut(&mut self, index: Factors) -> &mut Self::Output {
        &mut self.notes[index as usize]
    }
}

impl core::ops::Mul<LPR> for Triad {
    type Output = Self;

    fn mul(self, rhs: LPR) -> Self::Output {
        rhs.apply(&self)
    }
}

impl core::ops::MulAssign<LPR> for Triad {
    fn mul_assign(&mut self, rhs: LPR) {
        *self = rhs.apply(self);
    }
}

impl core::iter::IntoIterator for Triad {
    type Item = usize;
    type IntoIter = core::array::IntoIter<Self::Item, 3>;

    fn into_iter(self) -> Self::IntoIter {
        self.notes.into_iter()
    }
}

impl<'a> core::iter::IntoIterator for &'a Triad {
    type Item = &'a usize;
    type IntoIter = core::slice::Iter<'a, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.notes.iter()
    }
}

impl<'a> core::iter::IntoIterator for &'a mut Triad {
    type Item = &'a mut usize;
    type IntoIter = core::slice::IterMut<'a, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.notes.iter_mut()
    }
}

impl<T> PartialEq<T> for Triad
where
    T: AsRef<[usize]>,
{
    fn eq(&self, other: &T) -> bool {
        self.notes() == other.as_ref()
    }
}
