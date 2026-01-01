/*
    Appellation: consts <module>
    Created At: 2025.12.29:16:49:00
    Contrib: @FL03
*/
//! various constants defined and used throughout the library.

pub const OCTAVE_SIZE: usize = 12;
/// The C Major scale represented as an array of pitch class indices.
pub const C_MAJOR_SCALE: [usize; 7] = [0, 2, 4, 5, 7, 9, 11];
/// Defines the frequency of the A4 note in Hertz.
pub const A4_FREQUENCY: f64 = 440.0;
/// Defines the frequency of the C4 note in Hertz.
pub const C4_FREQUENCY: f64 = 261.625565;
