/*
    Appellation: transform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstmt_core as rstmt;
extern crate rstmt_neo as neo;

use neo::transform::LPR;
use neo::Triad;
use rstmt::{IntervalOps, Note};

use LPR::*;

#[test]
fn test_leading() {
    let c_major = Triad::major(Note::from_pitch(0));
    let next = c_major.transform(L);
    // Validate the resulting triad
    assert_eq!(next.root(), c_major.third());
    assert_eq!(next.third(), c_major.fifth());
    assert_eq!(next.fifth(), c_major.root().sub_semitone());
    // validate the transformation is invertible
    assert_eq!(c_major, next.transform(L));
}

#[test]
fn test_parallel() {
    let c_major = Triad::major(Note::from_pitch(0));
    let next = c_major.transform(P);
    // Validate the resulting triad
    assert_eq!(next.root(), c_major.root());
    assert_eq!(next.third(), c_major.third().sub_semitone());
    assert_eq!(next.fifth(), c_major.fifth());
    // validate the transformation is invertible
    assert_eq!(c_major, next.transform(P));
}

#[test]
fn test_relative() {
    let c_major = Triad::major(Note::from_pitch(0));
    let next = c_major.transform(R);
    // Validate the resulting triad
    assert_eq!(next.root(), c_major.fifth().add_tone());
    assert_eq!(next.third(), c_major.root());
    assert_eq!(next.fifth(), c_major.third());
    // validate the transformation is invertible
    assert_eq!(c_major, next.transform(R));
}
