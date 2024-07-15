/*
    Appellation: classes <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use rstmt::{Fifth, Third};

/// This trait denotes privately declared instances of different classes of triads.
/// Traditionally, triads have two primary classes: [major](Major) and [minor](Minor), however, there are
/// two additional classes: [augmented](Augmented) and [diminished](Diminished). This trait is used to determine
pub trait TriadKind {
    private!();

    fn class(&self) -> Triads {
        if self.is_augmented() {
            Triads::Augmented
        } else if self.is_diminished() {
            Triads::Diminished
        } else if self.is_major() {
            Triads::Major
        } else {
            Triads::Minor
        }
    }

    fn thirds(&self) -> (Third, Third) {
        use Third::*;
        match self.class() {
            Triads::Augmented => (Major, Major),
            Triads::Diminished => (Minor, Minor),
            Triads::Major => (Major, Minor),
            Triads::Minor => (Minor, Major),
        }
    }

    fn root_to_third(&self) -> Third {
        self.thirds().0
    }

    fn third_to_fifth(&self) -> Third {
        self.thirds().1
    }

    fn root_to_fifth(&self) -> Fifth {
        use Fifth::*;
        match self.class() {
            Triads::Augmented => Augmented,
            Triads::Diminished => Diminished,
            Triads::Major | Triads::Minor => Perfect,
        }
    }

    fn name(&self) -> &str;

    fn is_augmented(&self) -> bool {
        false
    }

    fn is_diminished(&self) -> bool {
        false
    }

    fn is_major(&self) -> bool {
        false
    }

    fn is_minor(&self) -> bool {
        false
    }
}

macro_rules! class {
    (@impl $name:ident::$call:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize),)]
        pub enum $name {}

        impl TriadKind for $name {
            seal!();

            fn name(&self) -> &str {
                stringify!($call)
            }

            paste::paste! {
                fn [<is_ $call>](&self) -> bool {
                    true
                }
            }
        }

        unsafe impl Send for $name {}

        unsafe impl Sync for $name {}
    };
    ($($name:ident::$call:ident),* $(,)?) => {
        $(
            class!(@impl $name::$call);
        )*
    };
}

class!(
    Augmented::augmented,
    Diminished::diminished,
    Major::major,
    Minor::minor
);

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::EnumIter,
    strum::EnumString,
    strum::VariantArray,
    strum::VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub enum Triads {
    Augmented,
    Diminished,
    #[default]
    Major,
    Minor,
}

impl Triads {
    pub fn augmented() -> Self {
        Triads::Augmented
    }

    pub fn diminished() -> Self {
        Triads::Diminished
    }

    pub fn major() -> Self {
        Triads::Major
    }

    pub fn minor() -> Self {
        Triads::Minor
    }

    pub fn thirds(&self) -> (Third, Third) {
        use Third::*;
        match self {
            Triads::Augmented => (Major, Major),
            Triads::Diminished => (Minor, Minor),
            Triads::Major => (Major, Minor),
            Triads::Minor => (Minor, Major),
        }
    }

    pub fn root_to_third(&self) -> Third {
        self.thirds().0
    }

    pub fn third_to_fifth(&self) -> Third {
        self.thirds().1
    }

    pub fn root_to_fifth(&self) -> Fifth {
        use Fifth::*;
        match self {
            Triads::Augmented => Augmented,
            Triads::Diminished => Diminished,
            Triads::Major | Triads::Minor => Perfect,
        }
    }
}
