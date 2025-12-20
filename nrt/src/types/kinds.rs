/*
    appellation: kinds <module>
    authors: @FL03
*/

macro_rules! triad_class {
    ($($(#[$meta:meta])* $vis:vis $i:ident $name:ident);* $(;)?) => {
        $(triad_class! { @impl $(#[$meta])* $vis $i $name })*
    };
    (@def $(#[$meta:meta])* $vis:vis enum $name:ident $({})? $(;)?) => {
        $(#[$meta])* $vis enum $name {}
    };
    (@def $(#[$meta:meta])* $vis:vis struct $name:ident $(;)?) => {
        $(#[$meta])* #[derive(Default)] $vis struct $name;
    };
    (@impl $(#[$meta:meta])* $vis:vis $repr:ident $name:ident) => {
        triad_class! {
            @def $(#[$meta])*
            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
            #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
            #[repr(transparent)]
            $vis $repr $name
        }

        unsafe impl Send for $name {}

        unsafe impl Sync for $name {}

        impl $crate::TriadKind for $name {
            seal! {}

            fn new() -> Self {
                Self::default()
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(stringify!($name))
            }
        }
    };
}

triad_class! {
    #[doc = "An initializable, transparent type defining the _major_ triad classification."]
    pub struct Major;
    #[doc = "An initializable, transparent type defining the _minor_ triad classification."]
    pub struct Minor;
    #[doc = "The [`Augmented`] triad kind defines a special class of triads composed of two major thirds."]
    pub struct Augmented;
    #[doc = "An initializable, transparent type defining the _diminished_ triad classification."]
    pub struct Diminished;
}
