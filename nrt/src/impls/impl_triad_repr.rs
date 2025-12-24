/*
    Appellation: impl_triad_repr <module>
    Created At: 2025.12.20:11:04:03
    Contrib: @FL03
*/
use crate::triad::TriadBase;

use crate::error::TriadError;
use crate::traits::RawTriad;
use crate::types::{LPR, TriadClass};
use num_traits::{Float, FromPrimitive};
use rstmt_core::{Augmented, Diminished, IntoAspn, Major, Minor, PitchMod};

impl<S> TriadBase<S, Augmented>
where
    S: RawTriad,
{
    /// returns a new instance of the [`TriadBase`] with the given chord and kind as an
    /// augmented triad.
    pub const fn augmented(chord: S) -> Self {
        TriadBase::new(chord, Augmented)
    }
}

impl<S> TriadBase<S, Diminished>
where
    S: RawTriad,
{
    /// returns a new instance of the [`TriadBase`] with the given chord and kind as a
    /// diminished triad.
    pub const fn diminished(chord: S) -> Self {
        TriadBase::new(chord, Diminished)
    }
}

impl<S> TriadBase<S, Major>
where
    S: RawTriad,
{
    /// returns a new instance of the [`TriadBase`] with the given chord and kind as a major
    /// triad.
    pub const fn major(chord: S) -> Self {
        TriadBase::new(chord, Major)
    }
}

impl<S> TriadBase<S, Minor>
where
    S: RawTriad,
{
    /// returns a new instance of the [`TriadBase`] with the given chord and kind as a minor
    /// triad.
    pub const fn minor(chord: S) -> Self {
        TriadBase::new(chord, Minor)
    }
}

impl TriadBase<[usize; 3], TriadClass, usize> {
    /// Create a new triad from a root pitch and class
    pub fn from_root<N>(root: N, class: TriadClass) -> Self
    where
        N: IntoAspn,
    {
        // all IntoNote implementations should* already compute pmod
        let note = root.into_aspn();
        let root = note.class();
        // generate the chord factors from the root and class
        let chord = [
            root,
            (root + class.root()).pmod(),
            (root + class.fifth()).pmod(),
        ];
        Self {
            class,
            chord,
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
    /// returns true if the pitches within the triad match its classification
    pub fn is_valid(&self) -> bool {
        self.class().validate(&self.chord())
    }
    // return the barycentric coordinates of the given note w.r.t the current triad
    pub fn barycentric<N, U>(&self, p: N) -> [U; 3]
    where
        N: IntoAspn,
        U: Float + FromPrimitive,
    {
        let note = p.into_aspn();
        let px = U::from_usize(note.class().pmod()).unwrap();
        let py = U::from_isize(*note.octave()).unwrap();
        let y = U::from_isize(*self.octave).unwrap();
        let [v0, v1, v2] = self.chord().map(|n| U::from_usize(n).unwrap());

        let d00 = v0 * v0 + y * y;
        let d01 = v0 * v1 + y * y;
        let d11 = v1 * v1 + y * y;
        let d20 = v2 * px + y * py;
        let d21 = v2 * v1 + y * y;

        let denom = d00 * d11 - d01 * d01;
        let a = (d11 * d20 - d01 * d21) / denom;
        let b = (d00 * d21 - d01 * d20) / denom;
        let c = U::one() - a - b;
        [a, b, c]
    }
    /// computes the centroid of the triad
    pub fn centroid<T>(&self) -> Option<[T; 2]>
    where
        T: Float + FromPrimitive,
    {
        let y = T::from_isize(*self.octave)?;
        let x = T::from_usize(self.chord().iter().sum())? / T::from_usize(self.chord().len())?;
        Some([x, y])
    }
    #[cfg(feature = "alloc")]
    /// returns the number of common tones between two triads
    pub fn common_tones(&self, other: &Self) -> Vec<usize> {
        self.chord()
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
        self.chord().contains(pitch.borrow())
    }
    /// returns some [`LPR`] transformation, iff they are within a single _step_ of one another
    /// otherwise, returns [`None`](Option::None).
    pub fn is_neighbor(&self, other: &Self) -> Option<LPR> {
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
}
