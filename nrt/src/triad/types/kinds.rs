/*
    appellation: kinds <module>
    authors: @FL03
*/

/// [`TriadKind`] is a trait that represents the kind of a triad in Neo-Riemannian theory.
pub trait TriadKind: 'static + Copy + Send + Sync + core::fmt::Debug + core::fmt::Display {
    private!();

    fn new() -> Self;
}

/*
 ************* Implementations *************
*/
impl TriadKind for super::Triads {
    seal!();

    fn new() -> Self {
        Self::default()
    }
}

macro_rules! impl_type_tag {
    ($($(#[doc $($doc:tt)*])? $vis:vis $i:ident $kind:ident);* $(;)?) => {
        $(
            impl_type_tag!(@impl $(#[doc $($doc)*])? $vis $i $kind);
        )*
    };
    (@impl $(#[doc $($doc:tt)*])? $vis:vis $i:ident $kind:ident) => {

        impl_type_tag!(@branch $(#[doc $($doc)*])? $vis $i $kind);
        impl_type_tag!(@impls $kind);
    };
    (@branch $(#[doc $($doc:tt)*])? $vis:vis enum $kind:ident) => {
        $(#[doc $($doc)*])?
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(
            feature = "serde",
            derive(serde_derive::Deserialize, serde_derive::Serialize),
        )]
        #[repr(transparent)]
        $vis enum $kind {};
    };
    (@branch $(#[doc $($doc:tt)*])? $vis:vis struct $kind:ident) => {
        $(#[doc $($doc)*])?
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(
            feature = "serde",
            derive(serde_derive::Deserialize, serde_derive::Serialize),
        )]
        #[repr(transparent)]
        $vis struct $kind;
    };
    (@impls $kind:ident) => {
        unsafe impl Send for $kind {}

        unsafe impl Sync for $kind {}

        impl $crate::triad::TriadKind for $kind {
            seal!();

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
