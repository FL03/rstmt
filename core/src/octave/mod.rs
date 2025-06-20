/*
    appellation: octave <module>
    authors: @FL03
*/
//! this module implements the [`Octave`] type, repsenting a particular octave in music.
#[doc(inline)]
pub use self::{octave::Octave, traits::*};

mod octave;

mod impls {
    pub mod impl_octave;
    pub mod impl_octave_ops;
    #[cfg(feature = "rand")]
    pub mod impl_octave_rand;
    pub mod impl_octave_repr;
}

mod traits {
    //! this module provides additional traits for the [`notes`](crate::notes) module.
    #[doc(inline)]
    pub use self::prelude::*;

    mod convert;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::convert::*;
    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::octave::Octave;
    #[doc(inline)]
    pub use super::traits::*;
}
