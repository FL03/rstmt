/*
    Appellation: pitches <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use triad_core::pitch::*;

fn assert_ok<T, E>(result: Result<T, E>) -> T
where
    E: core::fmt::Debug + core::fmt::Display,
{
    assert!(result.is_ok());
    result.unwrap()
}

#[test]
fn test_pitch() {
    let pitch = assert_ok(Pitches::try_from_value(12));
    let rhs = Natural::C;
    assert_eq!(pitch, rhs.as_class());
}
