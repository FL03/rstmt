/*
    Appellation: triad <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstmt_core as rstmt;
extern crate rstmt_neo as neo;

use neo::triad::{Major, Minor, Triad};
use rstmt::Note;

#[test]
fn test_traid() {
    let c_major = Triad::<Major>::new(Note::from_pitch(0));
    assert_eq!(c_major.root(), Note::from_pitch(0));
    assert_eq!(c_major.third(), Note::from_pitch(4));
    assert_eq!(c_major.fifth(), Note::from_pitch(7));

    let c_minor = Triad::<Minor>::new(Note::from_pitch(0));
    assert_eq!(c_minor.root(), Note::from_pitch(0));
    assert_eq!(c_minor.third(), Note::from_pitch(3));
    assert_eq!(c_minor.fifth(), Note::from_pitch(7));
}

#[test]
fn test_transform() {
    use neo::transform::LPR::*;
    let c_major = Triad::<Major>::new(Note::from_pitch(0));
    let next = dbg!(c_major.transform(P));
    assert_eq!(c_major.root(), next.root());
    assert_eq!(c_major.fifth(), next.fifth());
    
    assert_ne!(c_major.third(), next.third());
    assert_ne!(c_major.root_to_third(), next.root_to_third());
}
