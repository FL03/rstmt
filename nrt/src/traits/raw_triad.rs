/*
    Appellation: raw_triad <module>
    Created At: 2025.12.20:10:36:11
    Contrib: @FL03
*/
use crate::{RawTriadStore, RawTriadStoreMut};
/// The [`RawTriad`] trait defines the interface for all implementations of triads.
pub trait RawTriad<T>
where
    Self::Store<T>: RawTriadStore<Elem = T>,
{
    /// the type of the item stored in the triad.
    type Store<U>: RawTriadStore<Elem = U>;

    private! {}

    fn store(&self) -> &Self::Store<T>;

    fn store_mut(&mut self) -> &mut Self::Store<T>;
    /// returns a reference to the root note of the triad.
    fn root(&self) -> &T {
        self.store().root()
    }
    /// returns a mutable reference to the root note of the triad.
    fn root_mut(&mut self) -> &mut T
    where
        Self::Store<T>: RawTriadStoreMut,
    {
        self.store_mut().root_mut()
    }
    /// returns a reference to the third note of the triad.
    fn third(&self) -> &T {
        self.store().third()
    }
    /// returns a mutable reference to the third note of the triad.
    fn third_mut(&mut self) -> &mut T
    where
        Self::Store<T>: RawTriadStoreMut,
    {
        self.store_mut().third_mut()
    }
    /// returns a reference to the fifth note of the triad.
    fn fifth(&self) -> &T {
        self.store().fifth()
    }
    /// returns a mutable reference to the fifth note of the triad.
    fn fifth_mut(&mut self) -> &mut T
    where
        Self::Store<T>: RawTriadStoreMut,
    {
        self.store_mut().fifth_mut()
    }
}

/*
 ************* Implementations *************
*/

impl<T> RawTriad<T> for (T, T, T) {
    type Store<U> = (U, U, U);

    seal! {}

    fn store(&self) -> &Self::Store<T> {
        self
    }

    fn store_mut(&mut self) -> &mut Self::Store<T> {
        self
    }
}

impl<T> RawTriad<T> for [T; 3] {
    type Store<U> = [U; 3];

    seal! {}

    fn store(&self) -> &Self::Store<T> {
        self
    }

    fn store_mut(&mut self) -> &mut Self::Store<T> {
        self
    }
}
