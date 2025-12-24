/*
    Appellation: chord <module>
    Created At: 2025.12.24:14:33:29
    Contrib: @FL03
*/
//! this module defines the chord-related implementations, interfaces, and more in an effort
//! to generalize their behavior across various representations.
//!
#[doc(inline)]
pub use self::traits::*;

mod traits {
    #[doc(inline)]
    pub use self::raw_chord::*;

    mod raw_chord;
}
// prelude (local)
#[doc(hidden)]
pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::traits::*;
}
