/*
    Appellation: lpr <module>
    Contrib: @FL03
*/
use rstmt_nrt::{LPR, Triad};

#[test]
fn test_leading() {
    // c-major
    let triad = Triad::major(0);
    // e-minor
    let next = triad.transform(LPR::Leading);
    assert_eq!(next, Triad::minor(4));
    // invert
    assert_eq!(triad, next.leading());
}

#[test]
fn test_parallel() {
    // c-major
    let triad = Triad::major(0);
    // c-minor
    let next = triad.transform(LPR::Parallel);
    assert_eq!(next, Triad::minor(0));
    // invert
    assert_eq!(triad, next.parallel());
}

#[test]
fn test_relative() {
    // c-major
    let triad = Triad::major(0);
    // a-minor
    let next = triad.transform(LPR::Relative);
    assert_eq!(next, Triad::minor(9));
    // invert
    assert_eq!(triad, next.relative());
}
