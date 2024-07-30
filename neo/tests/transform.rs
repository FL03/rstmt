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
fn test_parallel_invert() {
    use LPR::P;

    let triad = Triad::<Major>::from_root(Note::from_pitch(0));
    // Validate the transformations invertability
    let pp = triad.chain([P, P]).unwrap();
    assert_eq!(triad.as_dyn(), pp);
    assert_eq!(triad, pp.cast());
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

#[test]
fn test_leading_invert() {
    use LPR::L;

    let triad = Triad::<Major>::from_root(Note::from_pitch(0));
    let ll = triad.chain([L, L]).unwrap();
    assert_eq!(triad.as_dyn(), ll);
    assert_eq!(triad, ll.cast());
}
