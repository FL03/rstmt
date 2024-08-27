/*
    Appellation: interval <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::IntervalLevel;

pub struct Interval<Q> {
    pub level: IntervalLevel,
    pub quality: Q,
    pub value: i8,
}

impl<Q> Interval<Q> {
    pub fn new(level: IntervalLevel, quality: Q, value: i8) -> Self {
        Self {
            level,
            quality,
            value,
        }
    }
}

pub struct Major;
pub struct Third<Q> {
    pub value: i8,
    pub quality: Q,
}

impl Third<Major> {
    pub fn major(value: i8) -> Self {
        debug_assert!(value.abs() % 4 == 0);
        Self {
            value,
            quality: Major,
        }
    }
}

pub struct IntervalQuality<T> {
    pub level: IntervalLevel,
    pub name: &'static str,
    pub value: Option<T>,
}
pub trait Quality<T> {
    type Level;

    fn level(&self) -> Self::Level;

    fn name(&self) -> &str;

    fn value(&self) -> T;
}
