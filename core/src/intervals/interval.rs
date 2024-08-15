/*
    Appellation: interval <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::IntervalLevel;

pub struct Interval<Q> {
    pub level: IntervalLevel,
    pub quality: Option<Q>,
    pub value: i8,
}

pub trait Quality<T> {
    type Group;

    fn group(&self) -> Self::Group;

    fn name(&self) -> &str;

    fn value(&self) -> T;
}

pub struct Variant<K, V> {
    pub group: K,
    pub name: &'static str,
    pub value: V,
}
