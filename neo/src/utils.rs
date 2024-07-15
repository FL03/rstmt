/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use num::traits::NumOps;
use rstmt::Intervals;

pub fn interval<A, B>(lhs: A, rhs: B) -> Intervals
where
    A: NumOps<B, i8>,
{
    Intervals::from_value(lhs - rhs)
}
