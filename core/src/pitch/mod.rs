/*
    appellation: pitch <module>
    authors: @FL03
*/
//! this module implements the [`Pitch`] type and its associated traits and types.
#[doc(inline)]
pub use self::{pitch_base::*, pitch_class::*, traits::*, wrapper::*};
// modules
mod pitch_base;
mod pitch_class;
mod wrapper;

mod traits {
    #[doc(inline)]
    pub use self::{convert::*, raw_pitch::*};

    mod convert;
    mod raw_pitch;
}
// prelude (local)
pub(crate) mod prelude {
    pub use super::pitch_base::*;
    pub use super::pitch_class::*;
    pub use super::traits::*;
    pub use super::wrapper::*;
}
/*
 ************* Types *************
*/
macro_rules! classes {
    (@impl $name:ident::<Natural>) => {
        paste::paste! {
            pub type $name = PitchClass<[<$name Note>], Natural>;
        }
    };
    (@impl $name:ident::<$kind:ident>) => {
        paste::paste! {
            pub type [<$name $kind>] = PitchClass<[<$name $kind Note>], $kind>;
        }
    };
    (@impl $name:ident::<$($kind:ident),+ $(,)?>) => {
        $(classes! { @impl $name::<$kind> })*
    };
    ($($name:ident::<$($K:ident),* $(,)?>),* $(,)?) => {
        $(classes! { @impl $name::<Natural, $($K),*> })*
    };
}

classes! {
    C::<Sharp>,
    D::<Flat, Sharp>,
    E::<Flat>,
    F::<Sharp>,
    G::<Flat, Sharp>,
    A::<Flat, Sharp>,
    B::<Flat>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pitch_class() {
        let c = C::default();
        assert_eq!(c.index(), 0);
    }
}
