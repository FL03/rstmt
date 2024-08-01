/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{builder::TriadBuilder, classes::*, store::TriadStore, triad::Triad};

pub(crate) mod builder;
pub(crate) mod classes;
pub(crate) mod store;
pub(crate) mod triad;

pub(crate) mod impls {
    pub mod impl_iter;
    pub mod impl_ops;
    pub mod impl_triad;
    pub mod impl_variants;
}

pub(crate) mod prelude {
    pub use super::builder::TriadBuilder;
    pub use super::classes::*;
    pub use super::triad::Triad;
}

use crate::TriadError;

pub trait Kind<T> {
    type Class: Classifier<T>;

    fn phantom() -> core::marker::PhantomData<T>
    where
        Self: Sized,
    {
        core::marker::PhantomData::<T>
    }

    fn name() -> &'static str
    where
        Self: Sized;
}

pub trait Classifier<T> {
    fn name(&self) -> &'static str;
}

impl<T> Classifier<T> for core::marker::PhantomData<T>
where
    T: Kind<T>,
{
    fn name(&self) -> &'static str {
        T::name()
    }
}

/// [IntoTriad] converts a type into a [Triad].
pub trait IntoTriad<K> {
    fn into_triad(self) -> Triad<K>;
}

pub trait TryIntoTriad<K> {
    fn try_into_triad(self) -> Result<Triad<K>, TriadError>;
}

pub trait TriadData {
    type Elem;

    fn root(&self) -> &Self::Elem;

    fn third(&self) -> &Self::Elem;

    fn fifth(&self) -> &Self::Elem;
}

/*
 ************* Implementations *************
*/
impl<T> TriadData for [T; 3] {
    type Elem = T;

    fn root(&self) -> &Self::Elem {
        &self[0]
    }

    fn third(&self) -> &Self::Elem {
        &self[1]
    }

    fn fifth(&self) -> &Self::Elem {
        &self[2]
    }
}

impl<T> TriadData for (T, T, T) {
    type Elem = T;

    fn root(&self) -> &Self::Elem {
        &self.0
    }

    fn third(&self) -> &Self::Elem {
        &self.1
    }

    fn fifth(&self) -> &Self::Elem {
        &self.2
    }
}
