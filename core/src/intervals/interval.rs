/*
    Appellation: interval <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::IntervalLevel;
use core::marker::PhantomData;

pub struct Interval<Q> {
    pub level: IntervalLevel,
    pub value: i8,
    pub(crate) _quality: PhantomData<Q>,
}

impl<Q> Interval<Q> {
    pub fn new(value: i8) -> Self {
        Self {
            level: IntervalLevel::from_i8(value),
            value,
            _quality: PhantomData::<Q>,
        }
    }
}

use num::traits::{FromPrimitive, NumOps, One, Zero};

pub trait MusicalTy: Copy + PartialEq + One + Zero + NumOps {}

pub enum Augmented {}

pub enum Diminished {}

pub enum Major {}

enum Minor {}

pub enum Perfect {}

pub struct Third<Q = Major, T = i8> {
    pub value: T,
    _quality: PhantomData<Q>,
}

impl<T> Third<Major, T> {
    pub fn major(value: T) -> Self
    where
        T: Copy + PartialEq + core::ops::Rem<Output = T> + FromPrimitive + Zero,
    {
        debug_assert!(value % T::from_u8(4).unwrap() == T::zero());
        Self {
            value,
            _quality: PhantomData::<Major>,
        }
    }
}

impl Third<Minor> {
    pub fn minor(value: i8) -> Self {
        debug_assert!(value % 3 == 0);
        Self {
            value,
            _quality: PhantomData::<Minor>,
        }
    }
}

impl<Q> Third<Q> {
    pub fn value(&self) -> i8 {
        self.value
    }
}

impl<Q> Level for Third<Q> {
    type Quality = Q;
}

pub trait Level {
    type Quality;
}

pub trait Quality<T> {
    type Level;

    fn name(&self) -> &str {
        core::any::type_name::<Self>()
    }

    fn value(&self) -> T;
}

impl Quality<i8> for Major {
    type Level = Third<Major>;

    fn value(&self) -> i8 {
        4
    }
}

impl Quality<i8> for Minor {
    type Level = Third<Minor>;

    fn value(&self) -> i8 {
        3
    }
}
