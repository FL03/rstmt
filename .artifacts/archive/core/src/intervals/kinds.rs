/*
    Appellation: kinds <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::variants::*;

pub(crate) mod variants;

use crate::{IntoPitch, Pitch};

/// [Intervals] enumerates the various intervals used within music theory.
/// The system considers a semitone to be the smallest interval, while the octave
/// describe the maximum distance between any two pitches.
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
    strum::EnumIs,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub enum Intervals {
    #[default]
    Semitone = 1,
    Tone = 2,
    Thirds(Third),
    Fourths(Fourth),
    Fifths(Fifth),
    Sevenths(Seventh),
    Octave = 12,
}

impl Intervals {
    pub fn dist(a: impl IntoPitch, b: impl IntoPitch) -> Self {
        Self::new(a.into_pitch().pymod(), b.into_pitch().pymod())
    }
    pub fn new<A, B, C>(lhs: A, rhs: B) -> Self
    where
        A: core::ops::Sub<B, Output = C>,
        C: Into<Intervals>,
    {
        (lhs - rhs).into()
    }
    /// Use the difference between two pitches to determine the interval.
    pub fn from_value(value: impl IntoPitch) -> Self {
        use Intervals::*;
        let Pitch(pitch) = value.into_pitch();
        match pitch.abs() % 12 {
            0 => Octave,
            1 => Semitone,
            2 => Tone,
            3 => Thirds(Third::Minor),
            4 => Thirds(Third::Major),
            5 => Fourths(Fourth::Perfect),
            6 => Fifths(Fifth::Diminished),
            7 => Fifths(Fifth::Perfect),
            8 => Fifths(Fifth::Augmented),
            9 => Sevenths(Seventh::Diminished),
            10 => Sevenths(Seventh::Minor),
            11 => Sevenths(Seventh::Major),
            _ => unreachable!("The pitch value is out of range."),
        }
    }
    /// A convenience method for constructing a new instance of the [Octave](Intervals::Octave) variant.
    pub fn octave() -> Self {
        Intervals::Octave
    }
    /// A convenience method for constructing a new instance of the [Semitone](Intervals::Semitone) variant.
    pub fn semitone() -> Self {
        Intervals::Semitone
    }
    /// A convenience method for constructing a new instance of the [Tone](Intervals::Tone) variant.
    pub fn tone() -> Self {
        Intervals::Tone
    }
    /// A convenience method for constructing a new variant, [`Thirds`](Intervals::Thirds).
    pub fn third(third: Third) -> Self {
        Intervals::Thirds(third)
    }
    /// A convenience method for constructing a new variant, [`Fourths`](Intervals::Fourths).
    pub fn fourth(fourth: Fourth) -> Self {
        Intervals::Fourths(fourth)
    }
    /// A convenience method for constructing a new variant, [`Fifths`](Intervals::Fifths).
    pub fn fifth(fifth: Fifth) -> Self {
        Intervals::Fifths(fifth)
    }
    /// A convenience method for constructing a new variant, [`Sevenths`](Intervals::Sevenths).
    pub fn seventh(seventh: Seventh) -> Self {
        Intervals::Sevenths(seventh)
    }
    /// Interpret the current interval as a pitch.
    pub fn as_pitch(&self) -> Pitch {
        Pitch::from(self.value())
    }
    /// Returns the name of the selected interval.
    pub fn name(&self) -> &str {
        self.as_ref()
    }
    /// Returns the value of the selected interval.
    pub fn value(&self) -> i8 {
        match *self {
            Intervals::Semitone => 1,
            Intervals::Tone => 2,
            Intervals::Thirds(third) => third as i8,
            Intervals::Fourths(fourth) => fourth as i8,
            Intervals::Fifths(fifth) => fifth as i8,
            Intervals::Sevenths(seventh) => seventh as i8,
            Intervals::Octave => 12,
        }
    }

    variant_constructors! {
        Intervals {
            Thirds.major_third::<Third::Major>(),
            Thirds.minor_third::<Third::Minor>(),
            Fourths.perfect_fourth::<Fourth::Perfect>(),
            Fifths.diminished_fifth::<Fifth::Diminished>(),
            Fifths.perfect_fifth::<Fifth::Perfect>(),
            Fifths.augmented_fifth::<Fifth::Augmented>(),
            Sevenths.diminished_seventh::<Seventh::Diminished>(),
            Sevenths.minor_seventh::<Seventh::Minor>(),
            Sevenths.major_seventh::<Seventh::Major>(),

        }
    }
}

macro_rules! impl_from_value {
    (@impl $name:ident::$variant:ident($T:ty)) => {
        impl From<$T> for $name {
            fn from(value: $T) -> Self {
                $name::$variant(value)
            }
        }
    };
    ($name:ident {$($variant:ident($T:ty)),* $(,)?}) => {
        $(
            impl_from_value!(@impl $name::$variant($T));
        )*
    };
}

impl<P> From<P> for Intervals
where
    P: IntoPitch,
{
    fn from(value: P) -> Self {
        Intervals::from_value(value)
    }
}

impl_from_value! {
    Intervals {
        Thirds(Third),
        Fourths(Fourth),
        Fifths(Fifth),
        Sevenths(Seventh),
    }
}
