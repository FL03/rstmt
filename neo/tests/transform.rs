/*
    Appellation: transform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstmt_core as rstmt;
extern crate rstmt_neo as neo;

use neo::transform::LPR;
use neo::triad::{Major, Triad};
use rstmt::Note;

#[test]
fn test_parallel() {
    use LPR::P;
    let c_major = Triad::<Major>::from_root(Note::from_pitch(0));
    let next = c_major.apply(P);
    // Validate the resulting triad
    assert_ne!(c_major.third(), next.third());
    assert_eq!(c_major.root(), next.root());
    assert_eq!(c_major.fifth(), next.fifth());

    // Compare the intervals between the two triads
    assert_ne!(c_major.root_to_third(), next.root_to_third());
}

#[test]
fn test_leading() {
    use LPR::L;
    let c_major = Triad::<Major>::from_root(Note::from_pitch(0));
    let next = c_major.apply(L);
    // Validate the resulting triad
    assert_eq!(c_major.third(), next.third());
    assert_ne!(c_major.root(), next.root());
    assert_eq!(c_major.fifth(), next.fifth());

    // Compare the intervals between the two triads
    assert_ne!(c_major.third_to_fifth(), next.third_to_fifth());
}
