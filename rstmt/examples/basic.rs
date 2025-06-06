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


fn _demo_modulo() -> rstmt::Result<()> {
    // Demonstrate the modulo operation
    let a = 17;
    let b = 12;

    // Using pymod to get the positive equivalent
    let result = a.pymod(b);
    println!("{} pymod {} = {}", a, b, result);
    // Using pymod to compute the modulo of a negative number
    let neg_result = (-a).pmod();
    println!("{} pmod = {}", a, neg_result);

    Ok(())
}
