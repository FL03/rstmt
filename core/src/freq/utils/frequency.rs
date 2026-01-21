/*
    Appellation: frequency <module>
    Created At: 2025.12.29:16:52:31
    Contrib: @FL03
*/
use num_traits::{Float, FromPrimitive};

/// Given some pitch class $`n`$ (in semitones) and an optional base frequency $`\beta`$ (in hertz),
/// calculate the corresponding frequency $`f`$.
///
/// ```math
/// F=\beta\cdot{2^{\frac{n}{12}}}
/// ```
pub fn get_frequency_of_pitch<T>(n: isize, root: T) -> T
where
    T: Float + FromPrimitive,
{
    let factor = T::from_f32(2f32.powf(n as f32 / 12f32))
        .expect("failed to convert into the configured type");
    root * factor
}
/// Compute the pitch class of a frequency (in hertz), using the formula:
///
/// ```math
/// n = 12\cdot\log_{2}(\frac{F}{\beta})
/// ```
pub fn classify_freq_with_scale<T>(freq: T, root: T) -> Option<isize>
where
    T: Float + FromPrimitive,
{
    let log_base = T::from_u8(2)?;
    let modulus = T::from_u8(12)?;
    // Calculate pitch class: round(12 * log2(frequency / 440))
    (modulus * (freq.abs() / root).log(log_base))
        .round()
        .to_isize()
}
