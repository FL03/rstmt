/*
    appellation: triad <module>
    authors: @FL03
*/
//! this module implements the triad
#[doc(inline)]
pub use self::{base_triad::TriadBase, std_triad::Triad, types::prelude::*};

pub mod base_triad;
pub mod std_triad;

mod impls {
    pub mod impl_triad_base;
}

pub mod types {
    //! this module implements additional types used to support the [`triad`](crate::triad)
    //! module.
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod class;
    pub mod factors;
    pub mod kinds;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::class::*;
        #[doc(inline)]
        pub use super::factors::*;
        #[doc(inline)]
        pub use super::kinds::*;
    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::std_triad::*;
    #[doc(inline)]
    pub use super::types::prelude::*;
    #[doc(inline)]
    pub use super::{RawStore, RawTriad, TriadStore,};
}

/// [`RawStore`] establishes a common interface for all containers used to store the notes
/// within a triad.
pub trait RawStore {
    /// the _type_ of the item stored in the container.
    type Item;

    private!();
}
/// A [`TriadStore`] is a particular type of [`RawStore`] used to store a specific (3) number
/// of elements
pub trait TriadStore: RawStore {
    /// returns a reference to the root note of the triad.
    fn root(&self) -> &Self::Item;
    /// returns a mutable reference to the root note of the triad.
    fn root_mut(&mut self) -> &mut Self::Item;
    /// returns a reference to the third note of the triad.
    fn third(&self) -> &Self::Item;
    /// returns a mutable reference to the third note of the triad.
    fn third_mut(&mut self) -> &mut Self::Item;
    /// returns a reference to the fifth note of the triad.
    fn fifth(&self) -> &Self::Item;
    /// returns a mutable reference to the fifth note of the triad.
    fn fifth_mut(&mut self) -> &mut Self::Item;
    /// update the root note of the triad and return a mutable reference to the store
    fn set_root(&mut self, root: Self::Item) -> &mut Self {
        *self.root_mut() = root;
        self
    }
    /// update the third note of the triad and return a mutable reference to the store
    fn set_third(&mut self, third: Self::Item) -> &mut Self {
        *self.third_mut() = third;
        self
    }
    /// update the fifth note of the triad and return a mutable reference to the store
    fn set_fifth(&mut self, fifth: Self::Item) -> &mut Self {
        *self.fifth_mut() = fifth;
        self
    }
}

/// The [`RawTriad`] trait defines the interface for all implementations of triads.
pub trait RawTriad<T> {
    /// the type of the item stored in the triad.
    type Store<_T>: TriadStore<Item = _T>;

    private!();

    fn store(&self) -> &Self::Store<T>;

    fn store_mut(&mut self) -> &mut Self::Store<T>;
    /// returns a reference to the root note of the triad.
    fn root(&self) -> &T {
        self.store().root()
    }
    /// returns a mutable reference to the root note of the triad.
    fn root_mut(&mut self) -> &mut T {
        self.store_mut().root_mut()
    }
    /// returns a reference to the third note of the triad.
    fn third(&self) -> &T {
        self.store().third()
    }
    /// returns a mutable reference to the third note of the triad.
    fn third_mut(&mut self) -> &mut T {
        self.store_mut().third_mut()
    }
    /// returns a reference to the fifth note of the triad.
    fn fifth(&self) -> &T {
        self.store().fifth()
    }
    /// returns a mutable reference to the fifth note of the triad.
    fn fifth_mut(&mut self) -> &mut T {
        self.store_mut().fifth_mut()
    }
}

/*
 ************* Implementations *************
*/

impl<T> RawStore for (T, T, T) {
    type Item = T;

    seal!();
}

impl<T> RawStore for [T; 3] {
    type Item = T;

    seal!();
}

impl<T> TriadStore for (T, T, T)
{
    fn root(&self) -> &T {
        &self.0
    }

    fn root_mut(&mut self) -> &mut T {
        &mut self.0
    }

    fn third(&self) -> &T {
        &self.1
    }

    fn third_mut(&mut self) -> &mut T {
        &mut self.1
    }

    fn fifth(&self) -> &T {
        &self.2
    }

    fn fifth_mut(&mut self) -> &mut T {
        &mut self.2
    }
}

impl<T> TriadStore for [T; 3]
{
    fn root(&self) -> &T {
        &self[0]
    }

    fn root_mut(&mut self) -> &mut T {
        &mut self[0]
    }

    fn third(&self) -> &T {
        &self[1]
    }

    fn third_mut(&mut self) -> &mut T {
        &mut self[1]
    }

    fn fifth(&self) -> &T {
        &self[2]
    }

    fn fifth_mut(&mut self) -> &mut T {
        &mut self[2]
    }
}

impl<T> RawTriad<T> for (T, T, T) {
    type Store<_U> = ( _U, _U, _U );

    seal!();

    fn store(&self) -> &Self::Store<T> {
        self
    }

    fn store_mut(&mut self) -> &mut Self::Store<T> {
        self
    }
}

impl<T> RawTriad<T> for [T; 3] {
    type Store<_U> = [_U; 3];

    seal!();

    fn store(&self) -> &Self::Store<T> {
        self
    }

    fn store_mut(&mut self) -> &mut Self::Store<T> {
        self
    }
}
