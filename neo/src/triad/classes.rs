/*
    Appellation: classes <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use rstmt::{Fifth, Note, Third};

/// This trait denotes privately declared instances of different classes of triads.
/// Traditionally, triads have two primary classes: [major](Major) and [minor](Minor), however, there are
/// two additional classes: [augmented](Augmented) and [diminished](Diminished). This trait is used to determine
pub trait TriadKind {
    private!();
    /// Returns a new instance of [PhantomData](core::marker::PhantomData);
    /// This method is the only possible constructor for these objects,
    /// a charecteristic enfored with 0-variant enum declarations.
    fn phantom() -> core::marker::PhantomData<Self> {
        core::marker::PhantomData::<Self>
    }

    fn is_valid(notes: &[Note; 3]) -> bool {
        Self::class().validate(notes)
    }

    fn class() -> Triads {
        if Self::is_major() {
            Triads::Major
        } else if Self::is_minor() {
            Triads::Minor
        } else if Self::is_augmented() {
            Triads::Augmented
        } else {
            Triads::Diminished
        }
    }

    fn thirds() -> (Third, Third) {
        Self::class().thirds()
    }

    fn root_to_third() -> Third {
        Self::thirds().0
    }

    fn third_to_fifth() -> Third {
        Self::thirds().1
    }

    fn root_to_fifth() -> Fifth {
        Self::class().root_to_fifth()
    }

    fn name() -> &'static str;

    fn is_augmented() -> bool {
        false
    }

    fn is_diminished() -> bool {
        false
    }

    fn is_major() -> bool {
        false
    }

    fn is_minor() -> bool {
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

            fn name() -> &'static str {
                stringify!($name)
            }

            paste::paste! {
                fn [<is_ $call>]() -> bool {
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

    pub fn validate(&self, notes: &[Note; 3]) -> bool {
        // the interval between the root and the third must be a third
        let rt = notes[1] - notes[0];
        // the interval between the third and the fifth must be a third
        let tf = notes[2] - notes[1];

        Third::try_from(rt.pitch()).is_ok() && Third::try_from(tf.pitch()).is_ok()
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
