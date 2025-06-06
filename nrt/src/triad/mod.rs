/*
    appellation: triad <module>
    authors: @FL03
*/
//! this module implements the triad
#[doc(inline)]
pub use self::{base::TriadBase, triad::Triad, types::prelude::*};

pub mod base;
#[allow(clippy::module_inception)]
mod triad;

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
    pub use super::triad::*;
    #[doc(inline)]
    pub use super::types::prelude::*;
    #[doc(inline)]
    pub use super::{RawStore, RawTriad, TriadicStore};
}

/// [`RawStore`] establishes a common interface for all containers used to store the notes
/// within a triad.
pub trait RawStore {
    /// the _type_ of the item stored in the container.
    type Item;

    private!();
}
/// The [`TriadicStore`] trait extends the [`RawStore`] trait to provide additional methods
pub trait TriadicStore: RawStore {
    fn root(&self) -> &Self::Item;

    fn root_mut(&mut self) -> &mut Self::Item;

    fn third(&self) -> &Self::Item;

    fn third_mut(&mut self) -> &mut Self::Item;

    fn fifth(&self) -> &Self::Item;

    fn fifth_mut(&mut self) -> &mut Self::Item;
}
/// The [`RawTriad`] trait defines the interface for all implementations of triads.
pub trait RawTriad {
    type Store<T>: RawStore<Item = T>;

    private!();
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

impl<T> TriadicStore for (T, T, T)
where
    T: Clone,
{
    fn root(&self) -> &Self::Item {
        &self.0
    }

    fn root_mut(&mut self) -> &mut Self::Item {
        &mut self.0
    }

    fn third(&self) -> &Self::Item {
        &self.1
    }

    fn third_mut(&mut self) -> &mut Self::Item {
        &mut self.1
    }

    fn fifth(&self) -> &Self::Item {
        &self.2
    }

    fn fifth_mut(&mut self) -> &mut Self::Item {
        &mut self.2
    }
}

impl<T> TriadicStore for [T; 3]
where
    T: Clone,
{
    fn root(&self) -> &Self::Item {
        &self[0]
    }

    fn root_mut(&mut self) -> &mut Self::Item {
        &mut self[0]
    }

    fn third(&self) -> &Self::Item {
        &self[1]
    }

    fn third_mut(&mut self) -> &mut Self::Item {
        &mut self[1]
    }

    fn fifth(&self) -> &Self::Item {
        &self[2]
    }

    fn fifth_mut(&mut self) -> &mut Self::Item {
        &mut self[2]
    }
}
