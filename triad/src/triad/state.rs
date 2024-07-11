/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::triad::Triads;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TriadState {
    pub(crate) class: Triads,
    pub(crate) hash: String,
}

impl TriadState {
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, strum::EnumIs, strum::VariantNames)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize), serde(rename_all = "lowercase"))]
#[repr(C)]
#[strum(serialize_all = "lowercase")]
pub enum BinaryState<Q = TriadState> {
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

    pub fn kind(&self) -> &str {
        match self {
            Self::Invalid(_) => "invalid",
            Self::Valid(_) => "valid",
        }
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
