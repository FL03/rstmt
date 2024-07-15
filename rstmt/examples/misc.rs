/*
    Appellation: misc <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use utils::pmod;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(11, utils::absmod(11, 12));
    assert_ne!(dbg!(pmod(-17)), dbg!(-17 % 12));

    Ok(())
}

mod utils {
    use num::traits::{FromPrimitive, Num, Signed};

    pub fn absmod<T>(value: T, m: T) -> T
    where
        T: Copy + Num + PartialOrd + Signed,
    {
        let val = value % m;
        if val >= T::zero() {
            return val;
        }
        ((val + m) % m).abs()
    }

    pub fn pmod<T>(value: T) -> T
    where
        T: Copy + FromPrimitive + Num + PartialOrd + Signed,
    {
        absmod(value, T::from_i8(12).unwrap())
    }
}
