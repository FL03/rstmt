/*
    appellation: ops <module>
    authors: @FL03
*/
extern crate rstmt_core as rstmt;

use rstmt::{PitchMod, PyMod};
use rstmt::freq::ScaleToFrequency;

#[test]
fn test_freq_convert() -> rstmt::Result<()> {
    let n: isize = -9; // C4;
    let base = ScaleToFrequency::new(440f64); // A4

    let freq = base.compute(n).unwrap();
    assert!((freq - 261.6255653005986).abs() < f64::EPSILON);
    let scale = base.from_scale_degree(freq).unwrap();
    assert_eq!(scale, n);

    Ok(())
}

#[test]
fn test_pymod() -> rstmt::Result<()> {
    let x = (-1).pymod(12);
    assert_eq!(x, 11);
    assert_ne!(x, -1);

    let a: isize = 17;
    let b: isize = 12;

    let c_pos = a.pymod(b);
    let c_neg = (-a).pymod(b);
    assert_eq!(c_pos, 5);
    assert_ne!(c_neg, -17 % 12);
    assert_eq!(c_neg, (-17).pmod());

    Ok(())
}
