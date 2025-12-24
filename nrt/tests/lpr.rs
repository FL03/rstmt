/*
    Appellation: lpr <module>
    Contrib: @FL03
*/
use rstmt_nrt::{LPR, Triad};

#[test]
fn test_leading() {
    let c_major = Triad::major(0);
    let next = c_major.transform(LPR::Leading);
    assert_eq! { next, Triad::minor(4) }
    assert_eq! { Some(c_major), next.leading().ok() }
}

#[test]
fn test_parallel() {
    let c_major = Triad::major(0);
    // c-minor
    let next = c_major.transform(LPR::Parallel);
    assert_eq! { next, Triad::minor(0) }
    // invert
    assert_eq! { Some(c_major), next.parallel().ok() }
}

#[test]
fn test_relative() {
    // c-major
    let c_major = Triad::major(0);
    let next = c_major.transform(LPR::Relative);
    assert_eq! { next, Triad::minor(9) }
    assert_eq! { Some(c_major), next.relative().ok() }
}
