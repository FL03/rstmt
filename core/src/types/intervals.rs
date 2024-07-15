/*
    Appellation: intervals <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{Notable, Pitch};
use num::traits::NumOps;

pub trait IntervalOps<Rhs = Self>: NumOps<Rhs, Pitch> {}

pub struct Interval<A, B> {
    pub lhs: A,
    pub rhs: B,
}

impl<A, B> Interval<A, B> {
    pub fn new(lhs: A, rhs: B) -> Self {
        Self { lhs, rhs }
    }
}

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
    pub fn from_value(value: impl Notable) -> Self {
        use Intervals::*;
        let pitch = value.pitch();
        match *pitch {
            1 => Semitone,
            2 => Tone,
            3..=4 => Thirds(Third::from(pitch.value())),
            5 => Fourths(Fourth::from(pitch.value())),
            6..=8 => Fifths(Fifth::from(pitch.value())),
            9..=12 => Sevenths(Seventh::from(pitch.value())),
            _ => panic!("Invalid interval value: {}", pitch.value()),
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

    pub fn value(&self) -> Pitch {
        let p = match *self {
            Intervals::Semitone => 1,
            Intervals::Tone => 2,
            Intervals::Thirds(third) => third as i8,
            Intervals::Fourths(fourth) => fourth as i8,
            Intervals::Fifths(fifth) => fifth as i8,
            Intervals::Sevenths(seventh) => seventh as i8,
        };
        Pitch(p)
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
