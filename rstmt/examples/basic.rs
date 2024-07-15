/*
    Appellation: misc <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use rstmt::{absmod, pmodulo};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(11, absmod(11, 12));
    assert_ne!(dbg!(pmodulo(-17)), dbg!(-17 % 12));
    println!("{}", pymod(17, -12));
    Ok(())
}

fn pymod<T>(lhs: T, rhs: T) -> T where T: Copy + num::Num + PartialOrd {
    let r = lhs % rhs;
    if (r < T::zero() && rhs > T::zero()) || (r > T::zero() && rhs < T::zero()) {
        r + rhs
    } else {
        r
    }
}