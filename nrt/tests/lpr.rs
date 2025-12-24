/*
    Appellation: lpr <module>
    Contrib: @FL03
*/
use rstmt_nrt::{LPR, Triad, TriadError};

#[test]
fn test_leading() -> Result<(), TriadError> {
    let c_major = Triad::major(0);
    let next = c_major.transform(LPR::Leading)?;
    assert_eq! { next, Triad::minor(4) }
    assert_eq! { c_major, next.leading()? }
    Ok(())
}

#[test]
fn test_parallel() -> Result<(), TriadError> {
    let c_major = Triad::major(0);
    // c-minor
    let next = c_major.transform(LPR::Parallel)?;
    assert_eq! { next, Triad::minor(0) }
    // invert
    assert_eq! { c_major, next.parallel()? }
    Ok(())
}

#[test]
fn test_relative() -> Result<(), TriadError> {
    // c-major
    let c_major = Triad::major(0);
    let next = c_major.transform(LPR::Relative)?;
    assert_eq! { next, Triad::minor(9) }
    assert_eq! { c_major, next.relative()? }
    Ok(())
}
