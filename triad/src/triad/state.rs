/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::triad::Triads;


#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TriadState {
    pub(crate) class: Triads,
    pub(crate) hash: String,
    
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub enum BinaryState<Q = String> {
    Invalid(Q),
    Valid(Q),
}

impl<Q> AsRef<Q> for BinaryState<Q> {
    fn as_ref(&self) -> &Q {
        match self {
            Self::Invalid(q) => q,
            Self::Valid(q) => q,
        }
    }
}

impl<Q> AsMut<Q> for BinaryState<Q> {
    fn as_mut(&mut self) -> &mut Q {
        match self {
            Self::Invalid(q) => q,
            Self::Valid(q) => q,
        }
    }
}

impl<Q: Default> Default for BinaryState<Q> {
    fn default() -> Self {
        Self::Invalid(Q::default())
    }
}

impl<Q> core::ops::Deref for BinaryState<Q> {
    type Target = Q;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}