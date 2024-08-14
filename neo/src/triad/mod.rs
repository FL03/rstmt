/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{kinds::*, triad::Triad};
#[doc(hidden)]
pub use self::{builder::TriadBuilder, utils::*};

pub(crate) mod builder;
pub(crate) mod kinds;
pub(crate) mod triad;

pub(crate) mod impls {
    pub mod impl_iter;
    pub mod impl_ops;
    pub mod impl_triad;
    pub mod impl_variants;
}

pub(crate) mod prelude {
    pub use super::builder::TriadBuilder;
    pub use super::kinds::*;
    pub use super::triad::Triad;
}

pub(crate) mod utils {
    use crate::error::TriadError;
    use rstmt::{Fifth, Note, Third};

    #[doc(hidden)]
    pub fn try_from_arr(notes: [Note; 3]) -> Result<(Note, Note, Note), TriadError> {
        use itertools::Itertools;
        for (&a, &b, &c) in notes.iter().circular_tuple_windows() {
            if Third::new(a, b).is_ok() && Third::new(b, c).is_ok() && Fifth::new(a, c).is_ok() {
                return Ok(dbg!((a, b, c)));
            } else {
                continue;
            }
        }
        Err(TriadError::InvalidInterval(
            "Failed to find the required relationships within the given notes...".into(),
        ))
    }
}

use crate::{Factors, TriadError};

/// [IntoTriad] converts a type into a [Triad].
pub trait IntoTriad {
    type Kind;

    fn into_triad(self) -> Triad<Self::Kind>;
}

pub trait TryIntoTriad {
    type Kind;

    fn try_into_triad(self) -> Result<Triad<Self::Kind>, TriadError>;
}

pub trait TriadData {
    type Elem;

    fn root(&self) -> &Self::Elem;

    fn root_mut(&mut self) -> &mut Self::Elem;

    fn third(&self) -> &Self::Elem;

    fn third_mut(&mut self) -> &mut Self::Elem;

    fn fifth(&self) -> &Self::Elem;

    fn fifth_mut(&mut self) -> &mut Self::Elem;

    fn get(&self, factor: Factors) -> &Self::Elem {
        match factor {
            Factors::Root => self.root(),
            Factors::Third => self.third(),
            Factors::Fifth => self.fifth(),
        }
    }

    fn get_mut(&mut self, factor: Factors) -> &mut Self::Elem {
        match factor {
            Factors::Root => self.root_mut(),
            Factors::Third => self.third_mut(),
            Factors::Fifth => self.fifth_mut(),
        }
    }
}

/*
 ************* Implementations *************
*/
impl<I> TriadData for I
where
    I: core::ops::Index<Factors, Output = rstmt::Note> + core::ops::IndexMut<Factors>,
{
    type Elem = rstmt::Note;

    fn root(&self) -> &Self::Elem {
        &self[Factors::Root]
    }

    fn root_mut(&mut self) -> &mut Self::Elem {
        &mut self[Factors::Root]
    }

    fn third(&self) -> &Self::Elem {
        &self[Factors::Third]
    }

    fn third_mut(&mut self) -> &mut Self::Elem {
        &mut self[Factors::Third]
    }

    fn fifth(&self) -> &Self::Elem {
        &self[Factors::Fifth]
    }

    fn fifth_mut(&mut self) -> &mut Self::Elem {
        &mut self[Factors::Fifth]
    }
}

impl<T> TriadData for [T; 3] {
    type Elem = T;

    fn root(&self) -> &Self::Elem {
        &self[0]
    }

    fn root_mut(&mut self) -> &mut Self::Elem {
        &mut self[0]
    }

    fn third(&self) -> &Self::Elem {
        &self[1]
    }

    fn third_mut(&mut self) -> &mut Self::Elem {
        &mut self[1]
    }

    fn fifth(&self) -> &Self::Elem {
        &self[2]
    }

    fn fifth_mut(&mut self) -> &mut Self::Elem {
        &mut self[2]
    }
}

impl<T> TriadData for (T, T, T) {
    type Elem = T;

    fn root(&self) -> &Self::Elem {
        &self.0
    }

    fn root_mut(&mut self) -> &mut Self::Elem {
        &mut self.0
    }

    fn third(&self) -> &Self::Elem {
        &self.1
    }

    fn third_mut(&mut self) -> &mut Self::Elem {
        &mut self.1
    }

    fn fifth(&self) -> &Self::Elem {
        &self.2
    }

    fn fifth_mut(&mut self) -> &mut Self::Elem {
        &mut self.2
    }
}
