/*
    Appellation: misc <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use rstmt::{absmod, pymod};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(11, absmod(11, 12));
    assert_ne!(dbg!(pymod(-17, 12)), dbg!(-17 % 12));
    println!("{}", pymod(17, -12));
    Ok(())
}

