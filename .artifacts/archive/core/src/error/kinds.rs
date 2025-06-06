/*
    Appellation: kinds <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(unused)]
pub use self::music::*;

mod music;

pub trait Kind {
    fn kind(&self) -> &str {
        core::any::type_name::<Self>()
    }
}

pub trait Err: core::fmt::Debug + core::fmt::Display {}

impl<T> Err for T where T: core::fmt::Debug + core::fmt::Display {}

pub enum ErrorKind {
    Known { kind: String },
    Unknown,
}

impl Kind for () {
    fn kind(&self) -> &str {
        "Unknown"
    }
}

impl<'a> Kind for &'a str {
    fn kind(&self) -> &str {
        self
    }
}

impl Kind for ErrorKind {
    fn kind(&self) -> &str {
        match self {
            Self::Known { kind } => kind,
            Self::Unknown => "Unknown",
        }
    }
}

pub mod msg {
    use super::Kind;

    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    pub struct ErrorMessage<K = (), S = String> {
        kind: K,
        msg: S,
    }

    impl<K, S> ErrorMessage<K, S> {
        pub fn new(kind: K, msg: S) -> Self {
            Self { kind, msg }
        }

        pub fn kind(&self) -> &K {
            &self.kind
        }

        pub const fn msg(&self) -> &S {
            &self.msg
        }
    }

    impl<S> core::fmt::Display for ErrorMessage<S>
    where
        S: core::fmt::Display,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", self.msg)
        }
    }

    unsafe impl<S> Send for ErrorMessage<S> {}

    unsafe impl<S> Sync for ErrorMessage<S> {}

    #[cfg(feature = "std")]
    impl<S> std::error::Error for ErrorMessage<S> where S: core::fmt::Debug + core::fmt::Display {}
}
