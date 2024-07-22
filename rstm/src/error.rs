/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::state::State;

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, strum::VariantNames, thiserror::Error,
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum FsmError<T = String> {
    #[error("[IndexError] Out of Bounds: {0}")]
    IndexOutOfBounds(String),
    #[error("[StateError] Invalid State: {0}")]
    InvalidState(String),
    #[error("[StateError] State not found: {current_state:?} with symbol: {read_symbol}")]
    StateNotFound {
        current_state: State<T>,
        read_symbol: char,
    },

    #[error("Transformation error: {0}")]
    TransformationError(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl<T> FsmError<T> {
    pub fn index_out_of_bounds(err: impl ToString) -> Self {
        FsmError::IndexOutOfBounds(err.to_string())
    }

    pub fn invalid_state(err: &str) -> Self {
        FsmError::InvalidState(err.to_string())
    }

    pub fn state_not_found(state: State<T>, char: char) -> Self {
        FsmError::StateNotFound {
            current_state: state,
            read_symbol: char,
        }
    }

    pub fn transformation_error(message: &str) -> Self {
        FsmError::TransformationError(message.to_string())
    }

    pub fn unknown(message: &str) -> Self {
        FsmError::Unknown(message.to_string())
    }
}
