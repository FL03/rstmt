/*
    Appellation: ops <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstmt_core as rstmt;

use rstmt::ops::{AbsMod, PitchMod};

#[test]
fn test_absmod() {
    let pairs = [(5, 12), (-5, 12), (0, 12), (12, 12), (13, 12), (-13, 12)];
    for (i, j) in pairs.iter() {
        assert_eq!(i.absmod(*j), i.pitchmod());
    }
}
