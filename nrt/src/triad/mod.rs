/*
    appellation: triad <module>
    authors: @FL03
*/
//! this module defines the [`Triad`] struct along with additional types and traits supporting
//! the representation of triads and their operations w.r.t. the neo-riemannian theory.
//!
//! ## Definition
//!
//! A triad is defined to be a chord, composed of three notes, each of which maintain certain
//! intervallic relationships with one another. More specifically, the distance between the
//! first and second as well as the second and third notes is defined to be a major or minor
//! third, whilst the distance between the first and third notes is some variant of a _fifth_.
//!
//! ## Examples
//!
//! ### _Example 1: Basic Usage_
//!
//! ```rust
//! use rstmt_nrt::Triad;
//!
//! // initialize a c-major triad: (0, 4, 7)
//! let triad = Triad::major(0);
//! // verify the composition
//! assert_eq!(triad.root(), &0);
//! assert_eq!(triad.third(), &4);
//! assert_eq!(triad.fifth(), &7);
//! assert!(triad.is_major());
//! ```
//!
//! ## Resources
//!
//! - [Continuous Transformations](https://www.mtosmt.org/issues/mto.04.10.3/mto.04.10.3.callender.pdf)
//! - [Neo-Riemannian Theory](https://en.wikipedia.org/wiki/Neo-Riemannian_theory)
//!
#[doc(inline)]
pub use self::{base_triad::TriadBase, std_triad::Triad, types::prelude::*};

pub mod base_triad;
mod std_triad;

mod impls {
    pub mod impl_triad_base;
}

mod types {
    //! this module implements additional types used to support the triad implementation
    #[doc(inline)]
    pub use self::prelude::*;

    mod class;
    mod factors;
    mod kinds;

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
    pub use super::{RawStore, RawTriad, TriadStore};
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
    type Store: TriadStore<Item = T>;

    private!();

    fn store(&self) -> &Self::Store;

    fn store_mut(&mut self) -> &mut Self::Store;
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

impl<T> TriadStore for (T, T, T) {
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

impl<T> TriadStore for [T; 3] {
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
    type Store = Self;
    seal!();

    fn store(&self) -> &Self::Store {
        self
    }

    fn store_mut(&mut self) -> &mut Self::Store {
        self
    }
}

impl<T> RawTriad<T> for [T; 3] {
    type Store = Self;

    seal!();

    fn store(&self) -> &Self::Store {
        self
    }

    fn store_mut(&mut self) -> &mut Self::Store {
        self
    }
}
