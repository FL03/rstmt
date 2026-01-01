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

pub const NATURAL_SYMBOL: char = '♮';
pub const SHARP_SYMBOL: char = '♯';
pub const FLAT_SYMBOL: char = '♭';
pub const DOUBLE_SHARP_SYMBOL: char = '𝄪';
pub const DOUBLE_FLAT_SYMBOL: char = '𝄫';

pub const NATURAL_PITCH_CLASSES: [&str; 7] = ["C", "D", "E", "F", "G", "A", "B"];

pub const SHARP_PITCH_CLASSES: [&str; 5] = ["C#", "D#", "F#", "G#", "A#"];

pub const FLAT_PITCH_CLASSES: [&str; 5] = ["Db", "Eb", "Gb", "Ab", "Bb"];

pub const ENHARMONIC_EQUIVALENTS: [(&str, &str); 10] = [
    ("C#", "Db"),
    ("D#", "Eb"),
    ("F#", "Gb"),
    ("G#", "Ab"),
    ("A#", "Bb"),
    ("Db", "C#"),
    ("Eb", "D#"),
    ("Gb", "F#"),
    ("Ab", "G#"),
    ("Bb", "A#"),
];