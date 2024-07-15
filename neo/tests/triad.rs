/*
    Appellation: triad <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstmt_core as rstmt;
extern crate rstmt_neo as neo;

use neo::triad::{Major, Triad};
use rstmt::Note;

#[test]
fn test_traid() {
    let triad = Triad::<Major>::new(Note::from_pitch(0));
    assert_eq!(triad.root(), Note::from_pitch(0));
    assert_eq!(triad.third(), Note::from_pitch(4));
    assert_eq!(triad.fifth(), Note::from_pitch(7));
}
