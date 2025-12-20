/*
    Appellation: impl_pitch_class <module>
    Created At: 2025.12.20:08:51:41
    Contrib: @FL03
*/
use crate::pitch::{PitchClass, PitchCls, PitchType};

impl<T, K> PitchClass<T, K>
where
    T: PitchCls<Tag = K>,
    K: PitchType,
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
        core::any::TypeId::of::<K>() == core::any::TypeId::of::<crate::pitch::Natural>()
    }
    /// returns true if the class is considered flat
    pub fn is_flat(&self) -> bool
    where
        K: 'static,
    {
        core::any::TypeId::of::<K>() == core::any::TypeId::of::<crate::pitch::Flat>()
    }
    /// returns true if the class is considered sharp
    pub fn is_sharp(&self) -> bool
    where
        K: 'static,
    {
        core::any::TypeId::of::<K>() == core::any::TypeId::of::<crate::pitch::Sharp>()
    }
}

impl<T, K> core::fmt::Debug for PitchClass<T, K>
where
    T: PitchCls<Tag = K> + core::fmt::Debug,
    K: PitchType,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.class)
    }
}

impl<T, K> core::fmt::Display for PitchClass<T, K>
where
    T: PitchCls<Tag = K> + core::fmt::Display,
    K: PitchType,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.class)
    }
}

impl<T, K> PitchCls for PitchClass<T, K>
where
    T: PitchCls<Tag = K>,
    K: PitchType,
{
    const IDX: usize = T::IDX;
    type Tag = K;

    seal! {}

    fn new() -> Self {
        Self {
            class: T::new(),
            _marker: core::marker::PhantomData,
        }
    }

    fn index(&self) -> usize {
        self.class.index()
    }
}

unsafe impl<T, K> Send for PitchClass<T, K>
where
    T: PitchCls<Tag = K>,
    K: PitchType,
{
}

unsafe impl<T, K> Sync for PitchClass<T, K>
where
    T: PitchCls<Tag = K>,
    K: PitchType,
{
}
