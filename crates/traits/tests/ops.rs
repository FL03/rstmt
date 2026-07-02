/*
    appellation: ops <module>
    authors: @FL03
*/

use rstmt_traits::{PitchMod, PyMod};

#[test]
fn test_pymod() {
    let x = (-1).pymod(12);
    assert_eq!(x, 11);
    assert_ne!(x, -1);

    let a: isize = 17;
    let b: isize = 12;

    let c_neg = (-a).pymod(b);
    assert_eq!(a.pymod(b), 5);
    assert_ne!(c_neg, -17 % 12);
    assert_eq!(c_neg, (-17).pmod());
}

#[test]
fn test_pitch_mod() {
    let a: isize = 17;
    let b: isize = 12;

    assert_eq!((-1).pmod(), 11);
    assert_eq!(a.pymod(b), 5);
}
