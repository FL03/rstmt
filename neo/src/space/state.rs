/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::binary::*;

use crate::triad::Triads;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TriadState {
    pub(crate) class: Triads,
    pub(crate) hash: String,
}

impl TriadState {
    pub fn new(class: Triads) -> Self {
        Self {
            class,
            hash: String::new(),
        }
    }
}

pub(crate) mod binary {

    #[derive(
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        strum::EnumDiscriminants,
        strum::EnumIs,
        strum::VariantNames,
    )]
    #[cfg_attr(
        feature = "serde",
        derive(serde::Deserialize, serde::Serialize),
        serde(rename_all = "lowercase"),
        strum_discriminants(
            derive(serde::Deserialize, serde::Serialize),
            serde(rename_all = "lowercase")
        )
    )]
    #[repr(C)]
    #[strum(serialize_all = "lowercase")]
    #[strum_discriminants(
        name(BinaryStates),
        derive(
            Ord,
            PartialOrd,
            strum::AsRefStr,
            strum::Display,
            strum::EnumCount,
            strum::EnumIs,
            strum::EnumIter,
            strum::EnumString,
            strum::VariantNames
        )
    )]
    pub enum BinaryState<Q = super::TriadState> {
        Invalid(Q),
        Valid(Q),
    }

    impl<Q> BinaryState<Q> {
        pub fn invalid(state: Q) -> Self {
            Self::Invalid(state)
        }

        pub fn valid(state: Q) -> Self {
            Self::Valid(state)
        }

        pub fn into_inner(self) -> Q {
            match self {
                Self::Invalid(q) => q,
                Self::Valid(q) => q,
            }
        }

        pub fn invalidate(self) -> Self {
            Self::Invalid(self.into_inner())
        }

        pub fn kind(&self) -> BinaryStates {
            match self {
                Self::Invalid(_) => BinaryStates::Invalid,
                Self::Valid(_) => BinaryStates::Valid,
            }
        }

        pub fn state(&self) -> (BinaryStates, &Q) {
            (self.kind(), self.as_ref())
        }
    }

    impl<Q> AsRef<Q> for BinaryState<Q> {
        fn as_ref(&self) -> &Q {
            match self {
                Self::Invalid(q) => q,
                Self::Valid(q) => q,
            }
        }
    }

    impl<Q> AsMut<Q> for BinaryState<Q> {
        fn as_mut(&mut self) -> &mut Q {
            match self {
                Self::Invalid(q) => q,
                Self::Valid(q) => q,
            }
        }
    }

    impl<Q: Default> Default for BinaryState<Q> {
        fn default() -> Self {
            Self::Invalid(Q::default())
        }
    }

    impl<Q> core::ops::Deref for BinaryState<Q> {
        type Target = Q;

        fn deref(&self) -> &Self::Target {
            self.as_ref()
        }
    }

    impl<Q> core::ops::DerefMut for BinaryState<Q> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.as_mut()
        }
    }

    impl<Q> core::fmt::Display for BinaryState<Q>
    where
        Q: core::fmt::Display,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", *self)
        }
    }
}
