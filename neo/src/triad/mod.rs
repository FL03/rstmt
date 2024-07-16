/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{builder::TriadBuilder, classes::*, triad::Triad};

pub(crate) mod builder;
pub(crate) mod classes;
pub(crate) mod triad;

pub(crate) mod prelude {
    pub use super::builder::TriadBuilder;
    pub use super::classes::*;
    pub use super::triad::Triad;
}

use rstmt::{Intervals, Notable};

pub trait IntoTriad<K> {
    fn into_triad(self) -> Triad<K>;
}

pub trait Triadic<N> where N: Notable {
    type Data: TriadData<Elem = N>;

    fn intervals(&self) -> impl Iterator<Item = Intervals>;

    fn kind(&self) -> Triads;

    fn notes(&self) -> &Self::Data;

    fn root(&self) -> &N {
        self.notes().root()
    }

    fn third(&self) -> &N {
        self.notes().third()
    }

    fn fifth(&self) -> &N {
        self.notes().fifth()
    }

    fn root_to_third(&self) -> Intervals {
        Intervals::from_value(self.third().pitch() - self.root().pitch())
    }

    fn third_to_fifth(&self) -> Intervals {
        Intervals::from_value(self.fifth().pitch() - self.third().pitch())
    }
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
