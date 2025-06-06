/*
    Appellation: pitch <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(unused)]
use crate::Octave;

use core::marker::PhantomData;

pub trait Opt {
    private!();
}

pub trait Class {
    type Opt: Opt;
}

pub trait Classifier<T> {
    type Class: Class<Opt = T>;
}

impl<C, T> Classifier<T> for PhantomData<C>
where
    C: Class<Opt = T>,
    T: Opt,
{
    type Class = C;
}

macro_rules! opts {
    (@impl $vis:vis enum $name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        $vis enum $name {}
    };
    (@impl $vis:vis struct $name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        $vis struct $name;
    };

    ($($vis:vis enum $name:ident),* $(,)?) => {
        $(
            opts!(@impl $vis enum $name);

            impl Opt for $name {
                seal!();
            }
        )*
    };
}

opts! {
    pub enum Natural,
    pub enum Sharp,
    pub enum Flat,
}

pub struct Note<const N: usize, K = Natural, T = i8> {
    pub(crate) octave: Octave,
    pub(crate) pitch: T,
    _kind: PhantomData<K>,
}

impl<const N: usize, K, T> Note<N, K, T> {
    pub fn new(pitch: T) -> Self {
        Self {
            octave: Octave::default(),
            pitch,
            _kind: PhantomData::<K>,
        }
    }

    pub const fn pitch(&self) -> &T {
        &self.pitch
    }

    pub fn pitch_mut(&mut self) -> &mut T {
        &mut self.pitch
    }

    pub fn into_inner(self) -> T {
        self.pitch
    }

    pub fn reset(&mut self)
    where
        T: Default,
    {
        self.pitch = <T>::default();
    }

    pub fn set(&mut self, pitch: T) {
        self.pitch = pitch;
    }

    pub fn is_flat(&self) -> bool
    where
        K: 'static,
    {
        core::any::TypeId::of::<Flat>() == core::any::TypeId::of::<K>()
    }

    pub fn is_natural(&self) -> bool
    where
        K: 'static,
    {
        core::any::TypeId::of::<Natural>() == core::any::TypeId::of::<K>()
    }

    pub fn is_sharp(&self) -> bool
    where
        K: 'static,
    {
        core::any::TypeId::of::<Sharp>() == core::any::TypeId::of::<K>()
    }
}

mod pitch {
    use super::Natural;
    use core::marker::PhantomData;

    // pub type D<K = Natural, T = i8> = PitchClass<2, K>;

    // pub type E<K = Natural, T = i8> = PitchClass<4, K>;

    // pub type F<K = Natural, T = i8> = PitchClass<5, K>;

    // pub type G<K = Natural, T = i8> = PitchClass<7, K>;

    // pub type A<K = Natural, T = i8> = PitchClass<9, K>;

    // pub type B<K = Natural, T = i8> = PitchClass<11, K>;

    pub struct PitchClass<const N: usize, K = Natural> {
        _kind: PhantomData<K>,
    }
}

mod notes {
    use super::*;

    pub struct C<K = Natural> {
        _kind: PhantomData<K>,
    }

    impl<K> C<K> {
        pub const INDEX: i8 = 0;

        pub fn new() -> Self {
            Self {
                _kind: PhantomData::<K>,
            }
        }

        pub fn is_natural(&self) -> bool
        where
            K: 'static,
        {
            core::any::TypeId::of::<Natural>() == core::any::TypeId::of::<K>()
        }

        pub fn is_sharp(&self) -> bool
        where
            K: 'static,
        {
            core::any::TypeId::of::<Sharp>() == core::any::TypeId::of::<K>()
        }
    }

    impl C<Natural> {
        pub fn natural() -> Self {
            Self {
                _kind: PhantomData::<Natural>,
            }
        }

        pub const fn get(&self) -> i8 {
            Self::INDEX
        }
    }

    impl C<Sharp> {
        pub fn sharp() -> Self {
            Self {
                _kind: PhantomData::<Sharp>,
            }
        }

        pub const fn get(&self) -> i8 {
            Self::INDEX + 1
        }
    }

    impl Class for C<Natural> {
        type Opt = Natural;
    }

    impl Class for C<Sharp> {
        type Opt = Sharp;
    }
}
