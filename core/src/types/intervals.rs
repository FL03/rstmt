/*
    Appellation: intervals <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::PitchTy;

use num::traits::NumOps;

pub trait IntervalOps<Rhs = Self>: NumOps<Rhs, PitchTy> {}

unit_enum! {
    rename: "lowercase";
    #[derive(Default)]
    pub enum Intervals {
        #[default]
        Semitone = 1,
        Tone = 2,
        Thirds(Third),
        Fourths(Fourth),
        Fifths(Fifth),
        Sevenths(Seventh),
    }
}

impl Intervals {
    pub fn from_value(value: PitchTy) -> Self {
        use Intervals::*;
        match value {
            1 => Semitone,
            2 => Tone,
            3..=4 => Thirds(Third::from(value)),
            5 => Fourths(Fourth::from(value)),
            6..=8 => Fifths(Fifth::from(value)),
            9..=12 => Sevenths(Seventh::from(value)),
            _ => panic!("Invalid interval value: {}", value),
        }
    }
    pub fn from_semitone() -> Self {
        Intervals::Semitone
    }

    pub fn from_tone() -> Self {
        Intervals::Tone
    }

    pub fn from_thirds(third: Third) -> Self {
        Intervals::Thirds(third)
    }

    pub fn from_fourths(fourth: Fourth) -> Self {
        Intervals::Fourths(fourth)
    }

    pub fn from_fifths(fifth: Fifth) -> Self {
        Intervals::Fifths(fifth)
    }

    pub fn from_sevenths(seventh: Seventh) -> Self {
        Intervals::Sevenths(seventh)
    }

    pub fn name(&self) -> &str {
        self.as_ref()
    }

    pub fn value(&self) -> PitchTy {
        match *self {
            Intervals::Semitone => 1,
            Intervals::Tone => 2,
            Intervals::Thirds(third) => third as i8,
            Intervals::Fourths(fourth) => fourth as i8,
            Intervals::Fifths(fifth) => fifth as i8,
            Intervals::Sevenths(seventh) => seventh as i8,
        }
    }
}

impl From<Third> for Intervals {
    fn from(third: Third) -> Self {
        Intervals::Thirds(third)
    }
}

impl From<Fourth> for Intervals {
    fn from(fourth: Fourth) -> Self {
        Intervals::Fourths(fourth)
    }
}

impl From<Fifth> for Intervals {
    fn from(fifth: Fifth) -> Self {
        Intervals::Fifths(fifth)
    }
}

impl From<Seventh> for Intervals {
    fn from(seventh: Seventh) -> Self {
        Intervals::Sevenths(seventh)
    }
}

interval! {
    default: Major;
    pub enum Third {
        Minor = 3,
        Major = 4,
    }
}

interval! {
    default: Perfect;
    pub enum Fourth {
        Perfect = 5,
    }
}

interval! {
    default: Perfect;
    pub enum Fifth {
        Diminished = 6,
        Perfect = 7,
        Augmented = 8,
    }
}

interval! {
    default: Diminished;
    pub enum Seventh {
        Diminished = 9,
        Minor = 10,
        Major = 11,
        Augmented = 12,
    }
}
