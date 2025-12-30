/*
    Appellation: kinds <module>
    Created At: 2025.12.29:18:04:49
    Contrib: @FL03
*/
/// The [`IntervalKind`] trait is a sealed marker trait used to define the allowable
/// representations, or kinds, of musical intervals.
pub trait IntervalKind {
    private! {}
}

macro_rules! interval_kind {
    (@def $(#[$meta:meta])* $vis:vis enum $name:ident) => {
        $(#[$meta])*
        $vis struct $name;
    };
    (@def $(#[$meta:meta])* $vis:vis struct $name:ident) => {
        $(#[$meta])*
        #[derive(Default)]
        $vis struct $name;
    };
    (@impl $(#[$meta:meta])* $vis:vis $obj:ident $name:ident) => {
        interval_kind! { @def $(#[$meta])*
            #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
            #[repr(transparent)]
            $vis $obj $name
        }

        impl IntervalKind for $name {
            seal! {}
        }
    };
    {$($(#[$meta:meta])* $vis:vis $kind:ident $name:ident),* $(,)?} => {
        $(interval_kind! { @impl $(#[$meta])* $vis $kind $name })*
    };
}

interval_kind! {
   pub struct Unison,
   pub struct Tone,
   pub struct Semitone,
   pub struct Third,
   pub struct Fourth,
   pub struct Fifth,
   pub struct Sixth,
   pub struct Seventh,
}
