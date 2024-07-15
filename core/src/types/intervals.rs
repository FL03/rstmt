/*
    Appellation: intervals <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{IntoPitch, Pitch};

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
    pub fn from_value(value: impl IntoPitch) -> Self {
        use Intervals::*;
        let pitch = value.into_pitch();
        match *pitch {
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
            12 => Sevenths(Seventh::Augmented),

            _ => panic!("Invalid interval value: {}", pitch.value()),
        }
    }
    pub fn semitone() -> Self {
        Intervals::Semitone
    }

    pub fn tone() -> Self {
        Intervals::Tone
    }

    pub fn third(third: Third) -> Self {
        Intervals::Thirds(third)
    }

    pub fn fourth(fourth: Fourth) -> Self {
        Intervals::Fourths(fourth)
    }

    pub fn fifth(fifth: Fifth) -> Self {
        Intervals::Fifths(fifth)
    }

    pub fn seventh(seventh: Seventh) -> Self {
        Intervals::Sevenths(seventh)
    }
    /// Interpret the current interval as a pitch.
    pub fn as_pitch(&self) -> Pitch {
        Pitch::from(self.value())
    }

    pub fn name(&self) -> &str {
        self.as_ref()
    }

    pub fn value(&self) -> i8 {
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

impl Fifth {
    pub fn from_thirds(lhs: Third, rhs: Third) -> Self {
        let value = lhs as i8 + rhs as i8;
        match value {
            6 => Fifth::Diminished,
            7 => Fifth::Perfect,
            8 => Fifth::Augmented,
            _ => panic!("Invalid fifth value: {}", value),
        }
    }
}
