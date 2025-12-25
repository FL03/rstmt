/*
    Appellation: rstmt-nrt <library>
    Contrib: @FL03
*/
//! This crate works to establish a solid foundation fo exploring the neo-riemannian theory,
//! providing implementations of the [`Triad`], its transformations [`LPR`], and the
//! generalized tonnetz ([`HyperTonnetz`]).
//!
//! ## Background
//!
//! Before diving into the implementation details, it is important to understand the concepts
//! at hand and the theory behind them.
//!
//! ### Neo-Riemannian Theory
//!
//! The neo-riemannian theory is a loose collection of musical theories focused on the triad.
//! Research in the field has been ongoing for over a century, culminating in the successful
//! generalization of the tonnetz, a geometric representation of the triad and its
//! transformations, into a single topological entity composed of individual simplices.
//!
//! ## Examples
//!
//! ### _Basic Usage_
//!
//! Create a C major triad and perform some basic operations.
//!
//! ```rust
//! use rstmt_nrt::Triad;
//!
//! // initialize a c-major triad: (0, 4, 7)
//! let mut triad = Triad::major(0);
//!
//! // verify the composition
//! assert_eq! { triad, [0, 4, 7] }
//! assert! { triad.is_major() }
//! // transform the triad using the parallel transformation
//! let tp = triad.parallel().unwrap();
//! // verify the transformation
//! assert_eq! { tp, [0, 3, 7] }
//! assert! { tp.is_minor() }
//! // invert the transformation by applying it again
//! assert_eq! { tp.parallel().unwrap(), triad }
//! ```
//!
//! ## Resources
//!
//! - [The Generalized Tonnetz](https://dmitri.mycpanel.princeton.edu/tonnetzes.pdf)
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::should_implement_trait,
    clippy::upper_case_acronyms
)]
#![cfg_attr(not(feature = "std"), no_std)]
// compiler check
#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error! { "either the \"std\" or \"alloc\" feature must be enabled" }
// external crates
#[cfg(feature = "alloc")]
extern crate alloc;
/// re-declare the external `rstmt_core` crate as `rstmt` for convenience
extern crate rstmt_core as rstmt;

#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
}
/// this module defines the standard error type, [`TriadError`], for the crate
pub mod error;
#[cfg(feature = "tonnetz")]
pub mod tonnetz;
pub mod transform;
pub mod triad;

mod impls {
    mod impl_triad_base;
    mod impl_triad_ext;
    mod impl_triad_repr;
}

pub mod iter {
    //! this module defines various iterators for traversing a tonnetz, chaining
    //! transformations and more.

    #[cfg(feature = "rayon")]
    #[doc(inline)]
    pub use self::parallel::ParIter;
    #[doc(inline)]
    pub use self::walker::Walk;

    #[cfg(feature = "rayon")]
    pub mod parallel;
    pub mod walker;

    pub(crate) mod prelude {
        #[cfg(feature = "rayon")]
        pub use super::parallel::*;
        pub use super::walker::*;
    }
}

mod traits {
    #[doc(inline)]
    pub use self::{raw_store::*, triad_kind::*, triadic::*};

    mod raw_store;
    mod triad_kind;
    mod triadic;
}

mod types {
    #[doc(inline)]
    pub use self::{kinds::*, factors::*, lpr::*};

    mod kinds;
    mod factors;
    mod lpr;
}
// re-exports
#[cfg(feature = "std")]
#[doc(inline)]
pub use self::transform::TriadNavigator;
#[doc(inline)]
pub use self::{error::*, iter::prelude::*, traits::*, triad::*, types::*};
#[cfg(feature = "tonnetz")]
#[doc(inline)]
pub use self::{tonnetz::HyperTonnetz, transform::MotionPlanner};
// prelude
#[doc(hidden)]
pub mod prelude {
    pub use crate::iter::prelude::*;
    #[cfg(feature = "tonnetz")]
    pub use crate::tonnetz::prelude::*;
    pub use crate::traits::*;
    #[cfg(feature = "alloc")]
    pub use crate::transform::prelude::*;
    pub use crate::triad::*;
    pub use crate::types::*;
}
