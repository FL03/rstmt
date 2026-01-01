/*
    Appellation: impl_pitch_class <module>
    Created At: 2025.12.20:08:51:41
    Contrib: @FL03
*/
use crate::freq::{Frequency, RawFrequency};
use crate::pitch::{Accidental, Flat, Natural, PitchClass, RawPitchClass, Sharp};
use num_traits::{Float, FromPrimitive};

impl<P, K> PitchClass<P, K>
where
    P: RawPitchClass<Tag = K>,
    K: Accidental,
{
    pub fn new() -> Self
    where
        P: Default,
    {
        Self {
            class: P::default(),
            kind: K::default(),
        }
    }
    /// returns a pointer to the class
    pub const fn as_ptr(&self) -> *const P {
        core::ptr::from_ref(self.get())
    }
    /// returns a mutable pointer to the class
    pub const fn as_mut_ptr(&mut self) -> *mut P {
        core::ptr::from_mut(self.get_mut())
    }
    pub fn into_frequency<T>(&self) -> Frequency<T>
    where
        P: RawPitchClass<Tag = K>,
        K: Accidental,
        T: RawFrequency + Float + FromPrimitive,
    {
        Frequency::from_pitch(self.get().index())
    }
    /// returns a reference to the defined class
    pub const fn get(&self) -> &P {
        &self.class
    }
    /// returns a mutable reference to the defined class
    pub const fn get_mut(&mut self) -> &mut P {
        &mut self.class
    }
    /// returns true if the class is considered natural
    pub fn is_natural(&self) -> bool
    where
        K: 'static,
    {
        Natural::of::<K>()
    }
    /// returns true if the class is considered flat
    pub fn is_flat(&self) -> bool
    where
        K: 'static,
    {
        Flat::of::<K>()
    }
    /// returns true if the class is considered sharp
    pub fn is_sharp(&self) -> bool
    where
        K: 'static,
    {
        Sharp::of::<K>()
    }
}

impl<N, K> core::fmt::Debug for PitchClass<N, K>
where
    N: RawPitchClass<Tag = K> + core::fmt::Debug,
    K: Accidental,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.is_natural() {
            write!(f, "{:?}", self.class)
        } else {
            write!(f, "{:?}[{:?}]", self.class, self.kind.name())
        }
    }
}

impl<N, K> core::fmt::Display for PitchClass<N, K>
where
    N: RawPitchClass<Tag = K> + core::fmt::Display,
    K: Accidental,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.class)
    }
}
