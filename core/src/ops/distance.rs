/*
    Appellation: distance <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// This trait is used for computing the distance between two objects;
/// The generality of the trait enables developers to implement it for
/// their algorithms of choice operating on two objects within the same
/// space.
///
/// Musically, intervals are considered to be the distance between any two
/// pitches or tones. The distance between two pitches is typically measured
/// in semitones (+/- 1) or tones (+/- 2).
///
/// Musically, the distance between two pitches is considered to be an interval.
/// Intervals are typically measured in semitones (+/- 1) or tones (+/- 2).
pub trait Distance<Rhs = Self> {
    type Output;

    fn distance(self, rhs: Rhs) -> Self::Output;
}
