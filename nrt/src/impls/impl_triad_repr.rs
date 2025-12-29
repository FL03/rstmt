/*
    Appellation: impl_triad_repr <module>
    Created At: 2025.12.20:11:04:03
    Contrib: @FL03
*/
use crate::triad::TriadBase;

use crate::traits::TriadRepr;
use crate::types::{LPR, TriadClass};
use num_traits::{Float, FromPrimitive, Num, ToPrimitive};
use rstmt_core::{Augmented, Diminished, Major, Minor, PitchMod};

impl<S, T> TriadBase<S, Augmented, T>
where
    S: TriadRepr<Elem = T>,
{
    /// returns a new instance of the [`TriadBase`] with the given chord and kind as an
    /// augmented triad.
    pub fn augmented(root: T) -> Self
    where
        T: Copy + FromPrimitive + core::ops::Add<Output = T> + PitchMod<Output = T>,
    {
        TriadBase::from_root_with_class(root, Augmented)
    }
}

impl<S, T> TriadBase<S, Diminished, T>
where
    S: TriadRepr<Elem = T>,
{
    /// returns a new instance of the [`TriadBase`] with the given chord and kind as a
    /// diminished triad.
    pub fn diminished(root: T) -> Self
    where
        T: Copy + FromPrimitive + core::ops::Add<Output = T> + PitchMod<Output = T>,
    {
        TriadBase::from_root_with_class(root, Diminished)
    }
}

impl<S, T> TriadBase<S, Major, T>
where
    S: TriadRepr<Elem = T>,
{
    /// returns a new instance of the [`TriadBase`] with the given chord and kind as a major
    /// triad.
    pub fn major(root: T) -> Self
    where
        T: Copy + FromPrimitive + core::ops::Add<Output = T> + PitchMod<Output = T>,
    {
        TriadBase::from_root_with_class(root, Major)
    }
}

impl<S, T> TriadBase<S, Minor, T>
where
    S: TriadRepr<Elem = T>,
{
    /// returns a new instance of the [`TriadBase`] with the given chord and kind as a minor
    /// triad.
    pub fn minor(root: T) -> Self
    where
        T: Copy + FromPrimitive + core::ops::Add<Output = T> + PitchMod<Output = T>,
    {
        TriadBase::from_root_with_class(root, Minor)
    }
}

impl<T> TriadBase<[T; 3], TriadClass, T>
where
    T: Copy + ToPrimitive + FromPrimitive + PitchMod<Output = T> + core::ops::Add<Output = T>,
{
    /// creates a new augmented triad from the given root
    pub fn augmented(root: T) -> Self {
        Self::from_root_with_class(root, TriadClass::Augmented)
    }
    /// creates a new diminished triad from the given root
    pub fn diminished(root: T) -> Self
    where
        T: Copy + FromPrimitive + core::ops::Add<Output = T> + PitchMod<Output = T>,
    {
        Self::from_root_with_class(root, TriadClass::Diminished)
    }
    /// Create a new major triad from the given root
    pub fn major(root: T) -> Self
    where
        T: Copy + FromPrimitive + core::ops::Add<Output = T> + PitchMod<Output = T>,
    {
        Self::from_root_with_class(root, TriadClass::Major)
    }
    /// creates a new minor triad from the given root
    pub fn minor(root: T) -> Self
    where
        T: Copy + FromPrimitive + core::ops::Add<Output = T> + PitchMod<Output = T>,
    {
        Self::from_root_with_class(root, TriadClass::Minor)
    }
    /// return the barycentric coordinates of the given note w.r.t the current triad
    pub fn barycentric<N, U>(&self, p: N) -> [U; 3]
    where
        T: Copy + ToPrimitive,
        N: rstmt_core::IntoAspn,
        U: Float + FromPrimitive,
    {
        let note = p.into_aspn();
        let px = U::from_usize(note.class().pmod()).unwrap();
        let py = U::from_isize(*note.octave()).unwrap();
        let y = U::from_isize(*self.octave).unwrap();
        let [v0, v1, v2] = self.chord().map(|n| U::from(n).unwrap());

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
    /// returns true if the pitches within the triad match its classification
    pub fn is_valid(&self) -> bool
    where
        T: Copy
            + PartialEq
            + FromPrimitive
            + ToPrimitive
            + PitchMod<Output = T>
            + core::ops::Sub<Output = T>,
    {
        self.class().validate(self.chord())
    }
    /// returns some [`LPR`] transformation, iff they are within a single _step_ of one another
    /// otherwise, returns [`None`](Option::None).
    pub fn is_neighbor(&self, other: &Self) -> Option<LPR>
    where
        T: Num,
    {
        LPR::iter().find(|&t| {
            let result = self.transform(t).expect("transformation failed");
            result == *other
        })
    }
}
impl<T> TriadBase<[T; 3], TriadClass, T>
where
    T: Copy + ToPrimitive + FromPrimitive + Num + PitchMod<Output = T>,
{
    // TODO: create a `Walk` iterator using the given transformations and
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
}

impl TriadBase<[usize; 3], TriadClass> {
    #[cfg(feature = "alloc")]
    /// creates an instance of the transformer for the current triad
    pub fn path_finder(
        &self,
    ) -> crate::transform::TriadNavigator<'_, [usize; 3], crate::TriadClass, usize> {
        crate::transform::TriadNavigator::new(self)
    }
    /// apply a single transformation to a triad in-place, mutating the current instance
    pub fn transform_inplace(&mut self, transform: LPR) {
        *self = self.transform(transform).expect("transformation failed");
    }
}
