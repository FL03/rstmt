/*
    Appellation: impl_pitch_class <module>
    Created At: 2025.12.20:08:51:41
    Contrib: @FL03
*/
use crate::pitch::{Accidental, Flat, Natural, PitchClass, PitchRepr, Sharp};

impl<T, K> PitchClass<T, K>
where
    T: PitchRepr<Tag = K>,
    K: Accidental,
{
    pub fn new() -> Self {
        Self {
            class: T::new(),
            _marker: core::marker::PhantomData::<K>,
        }
    }
    /// returns a pointer to the inner class
    pub const fn as_ptr(&self) -> *const T {
        core::ptr::from_ref(self.get())
    }
    /// returns a mutable pointer to the inner class
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        core::ptr::from_mut(self.get_mut())
    }
    /// returns a reference to the inner class
    pub const fn get(&self) -> &T {
        &self.class
    }
    /// returns a mutable reference to the inner class
    pub const fn get_mut(&mut self) -> &mut T {
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

impl<T, K> core::fmt::Debug for PitchClass<T, K>
where
    T: PitchRepr<Tag = K> + core::fmt::Debug,
    K: Accidental,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?} {:?}", self.class, self._marker.name())
    }
}

impl<T, K> core::fmt::Display for PitchClass<T, K>
where
    T: PitchRepr<Tag = K> + core::fmt::Display,
    K: Accidental,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.class)
    }
}

impl<T, K> AsRef<str> for PitchClass<T, K>
where
    T: PitchRepr<Tag = K>,
    K: Accidental,
{
    fn as_ref(&self) -> &str {
        self.class.as_ref()
    }
}

impl<S, K> core::ops::Deref for PitchClass<S, K>
where
    S: crate::pitch::PitchRepr<Tag = K>,
    K: crate::pitch::Accidental,
{
    type Target = S;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

unsafe impl<T, K> Send for PitchClass<T, K>
where
    T: PitchRepr<Tag = K>,
    K: Accidental,
{
}

unsafe impl<T, K> Sync for PitchClass<T, K>
where
    T: PitchRepr<Tag = K>,
    K: Accidental,
{
}

impl<T, K> PitchRepr for PitchClass<T, K>
where
    T: PitchRepr<Tag = K>,
    K: Accidental,
{
    const IDX: isize = T::IDX;
    type Tag = K;

    seal! {}

    fn new() -> Self {
        Self {
            class: T::new(),
            _marker: core::marker::PhantomData,
        }
    }

    fn value(&self) -> isize {
        self.class.value()
    }
}
