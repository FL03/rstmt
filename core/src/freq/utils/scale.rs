/*
    appellation: frequency <module>
    authors: @FL03
*/
use crate::PitchMod;
use num_traits::{Float, NumCast};
// For pitch-class modular arithmetic

/// Maps a frequency to the nearest pitch class in the C Major scale (C=0, D=2, E=4, F=5, G=7, A=9, B=11).
/// The input frequency is in Hz, and T is a floating-point type (e.g., f32, f64).
pub fn map_to_c_major<T: Float + NumCast + Copy>(frequency: T) -> Option<i32> {
    // Ensure frequency is positive
    if frequency <= T::zero() {
        return None;
    }

    // C Major scale pitch classes
    const C_MAJOR: &[i32] = &[0, 2, 4, 5, 7, 9, 11];

    // Reference frequency (A4 = 440 Hz)
    let ref_freq = T::from(440.0).unwrap();

    // Calculate pitch class: round(12 * log2(frequency / 440))
    let log2 = T::from(2.0).unwrap();
    let semitones = T::from(12.0).unwrap() * (frequency / ref_freq).log(log2);
    let pitch_class = semitones.round().to_i32()?.pmod();

    // Find the nearest pitch class in the C Major scale
    let nearest = C_MAJOR
        .iter()
        .min_by_key(|&&scale_pc| {
            // Calculate the absolute distance, considering octave wrapping
            let dist = (pitch_class - scale_pc).pmod();
            dist.min(12 - dist) // Handle wrapping (e.g., distance from 11 to 0)
        })
        .copied();

    nearest
}
