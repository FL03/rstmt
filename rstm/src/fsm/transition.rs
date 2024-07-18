/*
    Appellation: transition <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::fsm::State;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Transition<T = String> {
    pub(crate) current_state: State<T>,
    pub(crate) read_symbol: char,
    pub(crate) next_state: State,
    pub(crate) write_symbol: char,
    pub(crate) direction: char,
}