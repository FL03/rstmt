/*
    Appellation: pitches <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstmt_core as rstmt;

use rstmt::pitch::*;

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
    let pitch = Pitches::from(12);
    //  verify the pitch class is natural
    assert!(pitch.is_natural());
    //  verify the pitch class is a natural note C
    assert_eq!(pitch, Natural::C);
    // verify the modulo of the pitch class
    assert_eq!(pitch.get(), 0);
}

#[test]
fn test_class_parse() {
    use core::str::FromStr;
    let pitch = Pitches::from_str("c").unwrap();
    assert_eq!(pitch, Natural::C);
    let pitch = Pitches::from_str("c#").unwrap();
    assert_eq!(pitch, Sharp::C);
}
