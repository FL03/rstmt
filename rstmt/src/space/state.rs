/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::binary::BinaryState;

pub mod binary;

use crate::neo::triad::Triads;

pub trait BinState {
    fn is(&self) -> bool;
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct State<T, Q> {
    pub(crate) data: T,
    pub(crate) state: Q,
}

impl<T, Q> State<T, Q> {
    pub fn new(data: T, state: Q) -> Self {
        Self { data, state }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn state(&self) -> &Q {
        &self.state
    }
}
