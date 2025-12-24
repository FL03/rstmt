/*
    Appellation: raw_triad <module>
    Created At: 2025.12.20:10:36:11
    Contrib: @FL03
*/
use rstmt_core::RawChord;

/// The [`RawTriad`] trait is used to restrict and extend the [`RawChord`] trait to define valid  
/// representations of a triad.
pub trait RawTriad: RawChord {
    private! {}

    fn from_arr(arr: [Self::Elem; 3]) -> Self
    where
        Self: Sized;

    fn from_iter<I>(iter: I) -> Option<Self>
    where
        I: IntoIterator<Item = Self::Elem>,
        Self: Sized,
    {
        let mut it = iter.into_iter();
        let a = it.next()?;
        let b = it.next()?;
        let c = it.next()?;
        Some(Self::from_arr([a, b, c]))
    }
    fn len(&self) -> usize {
        3
    }
    /// returns a reference to the root note of the triad.
    fn root(&self) -> &Self::Elem;
    /// returns a reference to the third note of the triad.
    fn third(&self) -> &Self::Elem;
    /// returns a reference to the fifth note of the triad.
    fn fifth(&self) -> &Self::Elem;
}
/// The [`RawTriadMut`] trait is used to extend the [`RawTriad`] trait to provide mutable access
/// to the elements of a triad.
pub trait RawTriadMut: RawTriad {
    /// returns a mutable reference to the root note of the triad.
    fn root_mut(&mut self) -> &mut Self::Elem;
    /// returns a mutable reference to the third note of the triad.
    fn third_mut(&mut self) -> &mut Self::Elem;
    /// returns a mutable reference to the fifth note of the triad.
    fn fifth_mut(&mut self) -> &mut Self::Elem;
    /// update the root note of the triad and return a mutable reference to the store
    fn set_root(&mut self, root: Self::Elem) -> &mut Self {
        *self.root_mut() = root;
        self
    }
    /// update the third note of the triad and return a mutable reference to the store
    fn set_third(&mut self, third: Self::Elem) -> &mut Self {
        *self.third_mut() = third;
        self
    }
    /// update the fifth note of the triad and return a mutable reference to the store
    fn set_fifth(&mut self, fifth: Self::Elem) -> &mut Self {
        *self.fifth_mut() = fifth;
        self
    }
}

/*
 ************* Implementations *************
*/

impl<T> RawTriad for (T, T, T) {
    seal! {}

    fn from_arr([a, b, c]: [Self::Elem; 3]) -> Self
    where
        Self: Sized,
    {
        (a, b, c)
    }

    fn root(&self) -> &T {
        &self.0
    }

    fn third(&self) -> &T {
        &self.1
    }

    fn fifth(&self) -> &T {
        &self.2
    }
}

impl<T> RawTriadMut for (T, T, T) {
    fn root_mut(&mut self) -> &mut T {
        &mut self.0
    }
    fn third_mut(&mut self) -> &mut T {
        &mut self.1
    }

    fn fifth_mut(&mut self) -> &mut T {
        &mut self.2
    }
}

impl<T> RawTriad for [T; 3] {
    seal! {}

    fn from_arr(arr: [Self::Elem; 3]) -> Self
    where
        Self: Sized,
    {
        arr
    }

    fn root(&self) -> &T {
        &self[0]
    }

    fn third(&self) -> &T {
        &self[1]
    }

    fn fifth(&self) -> &T {
        &self[2]
    }
}

impl<T> RawTriadMut for [T; 3] {
    fn root_mut(&mut self) -> &mut T {
        &mut self[0]
    }

    fn third_mut(&mut self) -> &mut T {
        &mut self[1]
    }

    fn fifth_mut(&mut self) -> &mut T {
        &mut self[2]
    }
}
