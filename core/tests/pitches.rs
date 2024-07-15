/*
    Appellation: pitches <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstmt_core as rstmt;

use rstmt::pitch::*;

fn assert_ok<T, E>(result: Result<T, E>) -> T
where
    E: core::fmt::Debug + core::fmt::Display,
{
    assert!(result.is_ok());
    result.unwrap()
}

#[test]
fn test_pitch() {
    let pitch = Pitch::new(0);
    assert_eq!(pitch.class(), Pitch::new(12).class());
    let b = pitch + 1;

    assert_ne!(pitch, b);
    assert_eq!(b, Pitch::new(1));
}

#[test]
fn test_pitch_class() {
    let pitch = assert_ok(Pitches::try_from_value(12));
    let rhs = Natural::C;
    assert_eq!(pitch, rhs.as_class());
    assert_eq!(pitch.pitch(), 0);
}
