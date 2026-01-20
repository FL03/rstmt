/*
    Appellation: lpr <module>
    Contrib: @FL03
*/
use LPR::*;
use rstmt_nrt::{LPR, Triad};

#[test]
fn test_leading() {
    let c_major = Triad::major(0);
    assert_eq! { Leading.apply(&c_major), [4, 7, 11] }
    assert_eq! { Leading.apply(Leading.apply(&c_major)), c_major }
}

#[test]
fn test_parallel() {
    let c_major = Triad::major(0);
    assert_eq! { Parallel.apply(&c_major), [0, 3, 7] }
    assert_eq! { Parallel.apply(Parallel.apply(&c_major)), c_major }
}

#[test]
fn test_relative() {
    // c-major
    let c_major = Triad::major(0);
    assert_eq! { Relative.apply(&c_major), [9, 0, 4] }
    assert_eq! { Relative.apply(Relative.apply(&c_major)), c_major }
}
