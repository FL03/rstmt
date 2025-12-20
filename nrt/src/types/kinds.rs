/*
    appellation: kinds <module>
    authors: @FL03
*/

macro_rules! impl_type_tag {
    ($($(#[$meta:meta])* $vis:vis $i:ident $kind:ident);* $(;)?) => {
        $(
            impl_type_tag!(@impl $(#[$meta])* $vis $i $kind);
        )*
    };
    (@def $(#[$meta:meta])* $vis:vis enum $kind:ident) => {
        $(#[$meta])*
        $vis enum $kind {};
    };
    (@def $(#[$meta:meta])* $vis:vis struct $kind:ident) => {
        $(#[$meta])*
        #[derive(Default)]
        $vis struct $kind;
    };
    (@impl $(#[$meta:meta])* $vis:vis $i:ident $kind:ident) => {
        impl_type_tag! {
            @def
            $(#[$meta])*
            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
            #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
            #[repr(transparent)]
            $vis $i $kind
        }

        unsafe impl Send for $kind {}

        unsafe impl Sync for $kind {}

        impl $crate::TriadKind for $kind {
            seal! {}

            fn new() -> Self {
                Self::default()
            }
        }

        impl ::core::fmt::Display for $kind {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                // stringify the ident of the kind
                let tag = stringify!($kind);
                // write the tag in lowercase
                write!(f, "{}", tag.to_lowercase())
            }
        }
    };
}

impl_type_tag! {
    #[doc = "Major triad kind"]
    pub struct Major;
    #[doc = "Minor triad kind"]
    pub struct Minor;
    #[doc = "Augmented triad kind"]
    pub struct Augmented;
    #[doc = "Diminished triad kind"]
    pub struct Diminished;
}
