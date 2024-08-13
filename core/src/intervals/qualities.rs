/*
    Appellation: qualities <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
/// Musically speaking, an [interval quality](Quality) is used to identify the different versions of various musical
/// objects.
///
/// With respect to intervals, the quality is used to identify the different versions of the interval.
///     - [augmented](Augmented)
///     - [diminished](Diminished)
///     - [major](Major)
///     - [minor](Minor)
///     - [perfect](Perfect)
pub trait Quality {
    private!();

    fn phantom() -> PhantomData<Self> {
        PhantomData::<Self>
    }

    fn kind() -> IntervalQuality {
        match Self::name() {
            "Augmented" => IntervalQuality::augmented(),
            "Diminished" => IntervalQuality::diminished(),
            "Major" => IntervalQuality::major(),
            "Minor" => IntervalQuality::minor(),
            "Perfect" => IntervalQuality::perfect(),
            _ => unreachable!(),
        }
    }

    fn name() -> &'static str {
        core::any::type_name::<Self>()
    }

    fn is_augmented() -> bool {
        false
    }

    fn is_diminished() -> bool {
        false
    }

    fn is_perfect() -> bool {
        false
    }

    fn is_major() -> bool {
        false
    }

    fn is_minor() -> bool {
        false
    }
}

macro_rules! quality {
    (@impl $name:ident::$call:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize),)]
        pub enum $name {}

        impl Quality for $name {
            seal!();

            fn kind() -> IntervalQuality {
                IntervalQuality::$call()
            }

            fn name() -> &'static str {
                stringify!($call)
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
            quality!(@impl $name::$call);
        )*
    };
}

quality!(
    Augmented::augmented,
    Diminished::diminished,
    Major::major,
    Minor::minor,
    Perfect::perfect
);

use core::marker::PhantomData;

/// This enum provides a more streamlined means of working with the implemented [Quality] types.
/// Primarily, it provides simple creation routines for otherwise unitializable types.
#[derive(
    Clone,
    Copy,
    Debug,
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
    strum::VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[non_exhaustive]
#[strum(serialize_all = "lowercase")]
pub enum IntervalQuality {
    Augmented(PhantomData<Augmented>),
    Diminished(PhantomData<Diminished>),
    Major(PhantomData<Major>),
    Minor(PhantomData<Minor>),
    Perfect(PhantomData<Perfect>),
}

impl IntervalQuality {
    pub fn new<Q: Quality>() -> Self {
        Q::kind()
    }
    pub fn augmented() -> Self {
        IntervalQuality::Augmented(PhantomData)
    }

    pub fn diminished() -> Self {
        IntervalQuality::Diminished(PhantomData)
    }

    pub fn major() -> Self {
        IntervalQuality::Major(PhantomData)
    }

    pub fn minor() -> Self {
        IntervalQuality::Minor(PhantomData)
    }

    pub fn perfect() -> Self {
        IntervalQuality::Perfect(PhantomData)
    }

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "Augmented" | "augmented" => Some(IntervalQuality::augmented()),
            "Diminished" | "diminished" => Some(IntervalQuality::diminished()),
            "Major" | "major" => Some(IntervalQuality::major()),
            "Minor" | "minor" => Some(IntervalQuality::minor()),
            "Perfect" | "perfect" => Some(IntervalQuality::perfect()),
            _ => None,
        }
    }
}
