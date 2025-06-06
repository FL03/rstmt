/*
    Appellation: misc <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use rstmt::{PitchMod, PyMod};

fn main() -> rstmt::Result<()> {
    assert_eq!((-1).pymod(12), 11);
    assert_ne!((-17).pymod(12), -17 % 12);
    assert_eq!((-17).pymod(12), (-17).pmod());

    let py17: isize = 17.pymod(-12);
    println!("{}", py17);
    Ok(())
}
