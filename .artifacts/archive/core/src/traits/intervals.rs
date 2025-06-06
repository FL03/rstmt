/*
    Appellation: intervals <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::intervals::{Intervals, IntoInterval, Seventh};

/// [IntervalOps] is a trait for objects capable of basic arithmetic with intervals.
///
///
///
pub trait IntervalOps {
    type Output;
    /// Add an interval to the given object
    fn add_interval<I: IntoInterval>(&self, interval: I) -> Self::Output;
    /// Subtract an interval from the given object
    fn sub_interval<I: IntoInterval>(&self, interval: I) -> Self::Output;

    fn add_semitone(&self) -> Self::Output {
        self.add_interval(Intervals::Semitone)
    }

    fn sub_semitone(&self) -> Self::Output {
        self.sub_interval(Intervals::Semitone)
    }

    fn add_tone(&self) -> Self::Output {
        self.add_interval(Intervals::Tone)
    }

    fn sub_tone(&self) -> Self::Output {
        self.sub_interval(Intervals::Tone)
    }

    fn add_major_third(&self) -> Self::Output {
        self.add_interval(Intervals::major_third())
    }

    fn sub_major_third(&self) -> Self::Output {
        self.sub_interval(Intervals::major_third())
    }

    fn add_minor_third(&self) -> Self::Output {
        self.add_interval(Intervals::minor_third())
    }

    fn sub_minor_third(&self) -> Self::Output {
        self.sub_interval(Intervals::minor_third())
    }

    fn add_perfect_fifth(&self) -> Self::Output {
        self.add_interval(Intervals::perfect_fifth())
    }

    fn sub_perfect_fifth(&self) -> Self::Output {
        self.sub_interval(Intervals::perfect_fifth())
    }

    fn add_augmented_fifth(&self) -> Self::Output {
        self.add_interval(Intervals::augmented_fifth())
    }

    fn sub_augmented_fifth(&self) -> Self::Output {
        self.sub_interval(Intervals::augmented_fifth())
    }

    fn add_diminished_fifth(&self) -> Self::Output {
        self.add_interval(Intervals::diminished_fifth())
    }

    fn sub_diminished_fifth(&self) -> Self::Output {
        self.sub_interval(Intervals::diminished_fifth())
    }

    fn add_augmented_seventh(&self) -> Self::Output {
        self.add_interval(Seventh::Augmented)
    }

    fn sub_augmented_seventh(&self) -> Self::Output {
        self.sub_interval(Seventh::Augmented)
    }

    fn add_diminished_seventh(&self) -> Self::Output {
        self.add_interval(Seventh::Diminished)
    }

    fn sub_diminished_seventh(&self) -> Self::Output {
        self.sub_interval(Seventh::Diminished)
    }

    fn add_major_seventh(&self) -> Self::Output {
        self.add_interval(Seventh::Major)
    }

    fn sub_major_seventh(&self) -> Self::Output {
        self.sub_interval(Seventh::Major)
    }

    fn add_minor_seventh(&self) -> Self::Output {
        self.add_interval(Seventh::Minor)
    }

    fn sub_minor_seventh(&self) -> Self::Output {
        self.sub_interval(Seventh::Minor)
    }

    fn add_octave(&self) -> Self::Output {
        self.add_interval(Intervals::Octave)
    }

    fn sub_octave(&self) -> Self::Output {
        self.sub_interval(Intervals::Octave)
    }
}

/*
 ************* Implementations *************
*/
impl IntervalOps for crate::Note {
    type Output = crate::Note;

    fn add_interval<I: IntoInterval>(&self, interval: I) -> Self::Output {
        self + interval.into_interval()
    }

    fn sub_interval<I: IntoInterval>(&self, interval: I) -> Self::Output {
        self - interval.into_interval()
    }
}
