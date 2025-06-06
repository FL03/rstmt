/*
    appellation: ops <module>
    authors: @FL03
*/
extern crate rstmt_core as rstmt;

use rstmt::{PitchMod, PyMod};

#[test]
fn test_pymod() -> rstmt::Result<()> {
    let x = (-1).pymod(12);
    assert_eq!(x, 11);
    assert_ne!(x, -1 % 12);

    let a: isize = 17;
    let b: isize = 12;

    let c_pos = a.pymod(b);
    let c_neg = (-a).pymod(b);
    assert_eq!(c_pos, 5);
    assert_ne!(c_neg, -17 % 12);
    assert_eq!(c_neg, (-17).pmod());

    Ok(())
}
