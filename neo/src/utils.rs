/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use num::traits::NumOps;
use rstmt::Interval;

pub fn interval<A, B>(lhs: A, rhs: B) -> Interval
where
    A: NumOps<B, i8>,
{
    Interval::from_value(lhs - rhs)
}
