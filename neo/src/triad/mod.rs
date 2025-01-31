/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{kinds::*, triad::Triad};

pub(crate) mod kinds;
pub(crate) mod triad;

pub(crate) mod prelude {
    pub use super::kinds::*;
    pub use super::triad::Triad;
}

pub(crate) mod utils {
    use crate::error::NeoError;
    use rstmt::{Fifth, Note, Third};

    #[doc(hidden)]
    pub(super) fn try_from_arr(notes: [Note; 3]) -> Result<(Note, Note, Note), NeoError> {
        use itertools::Itertools;
        for (&a, &b, &c) in notes.iter().circular_tuple_windows() {
            if Third::new(a, b).is_ok() && Third::new(b, c).is_ok() && Fifth::new(a, c).is_ok() {
                return Ok(dbg!((a, b, c)));
            } else {
                continue;
            }
        }
        Err(NeoError::invalid_triad(
            "Failed to find the required relationships within the given notes...",
        ))
    }
}

