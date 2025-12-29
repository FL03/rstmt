/*
    Appellation: raw_chord <module>
    Created At: 2025.12.23:17:32:04
    Contrib: @FL03
*/
use rspace_traits::RawSpace;
/// The [`RawChord`] trait works to define a basic interface shared by all compatible
/// reprsentations of a chord. Since a chord is essentially a sequence of pitches, the trait
/// captures this behavior through association with an element type.
pub trait RawChord: RawSpace {
    /// returns a slice representation of the chord.
    fn as_slice(&self) -> &[Self::Elem];
    /// returns the number of elements in the chord representation.
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// The [`RawChordMut`] trait extends the [`RawChord`] trait to provide mutable access to the
/// underlying elements of the chord representation.
pub trait RawChordMut: RawSpace {
    /// returns a mutable slice representation of the chord.
    fn as_mut_slice(&mut self) -> &mut [Self::Elem];
}

pub trait ChordRepr: RawSpace {}
/// The [`RawChordIter`] trait extends the [`RawChord`] trait to provide an iterator over
/// the elements of the chord representation.
pub trait RawChordIter: RawSpace {
    type Iter<'b>: Iterator<Item = &'b Self::Elem>
    where
        Self::Elem: 'b,
        Self: 'b;

    fn iter(&self) -> Self::Iter<'_>;
}

/*
 ************* Implementations *************
*/

impl<C, T> RawChord for &C
where
    C: RawChord<Elem = T>,
{
    fn as_slice(&self) -> &[T] {
        C::as_slice(*self)
    }

    fn len(&self) -> usize {
        C::len(*self)
    }
}

impl<C, T> RawChord for &mut C
where
    C: RawChord<Elem = T>,
{
    fn as_slice(&self) -> &[T] {
        C::as_slice(*self)
    }

    fn len(&self) -> usize {
        C::len(*self)
    }
}

impl<T> RawChord for (T, T, T) {
    fn as_slice(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self as *const (T, T, T) as *const T, 3) }
    }

    fn len(&self) -> usize {
        3
    }
}

impl<T> RawChord for [T] {
    fn as_slice(&self) -> &[T] {
        self
    }

    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> RawChord for &[T] {
    fn as_slice(&self) -> &[T] {
        self
    }

    fn len(&self) -> usize {
        (*self).len()
    }
}

impl<T> RawChord for &mut [T] {
    fn as_slice(&self) -> &[T] {
        self
    }

    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<T> RawChordMut for [T] {
    fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }
}

impl<T> RawChordMut for &mut [T] {
    fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }
}

impl<const N: usize, T> RawChord for [T; N] {
    fn as_slice(&self) -> &[T] {
        self
    }

    fn len(&self) -> usize {
        N
    }
}

impl<const N: usize, T> RawChordMut for [T; N] {
    fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::{RawChord, RawChordMut};
    use alloc::vec::Vec;

    impl<T> RawChord for Vec<T> {
        fn as_slice(&self) -> &[T] {
            self.as_slice()
        }

        fn len(&self) -> usize {
            self.len()
        }
    }

    impl<T> RawChordMut for Vec<T> {
        fn as_mut_slice(&mut self) -> &mut [T] {
            self.as_mut_slice()
        }
    }
}
