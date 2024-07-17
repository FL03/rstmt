/*
    Appellation: pure <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// A pure tone is a sine-wave of a constant amplitude, frequency, and phase shift.
pub struct PureTone<T = f64> {
    amplitude: T,
    freq: T,
    phase: T,
}
