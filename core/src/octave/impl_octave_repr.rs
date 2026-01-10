/*
    appellation: impl_octave_repr <module>
    authors: @FL03
*/
use crate::octave::Octave;

impl<T> Octave<&T>
{
    /// returns a new instance of the [`Octave`] with a cloned instance of the current value.alloc
    pub fn cloned(&self) -> Octave<T>
    where
        T: Clone,
    {
        Octave(self.0.clone())
    }
    /// returns a new instance of the [`Octave`] with a copied instance of the current value
    pub const fn copied(&self) -> Octave<T>
    where
        T: Copy,
    {
        Octave(*self.0)
    }
}

impl<T> Octave<&mut T>
{
    /// returns a new instance of the [`Octave`] with a cloned instance of the current value.alloc
    pub fn cloned(&self) -> Octave<T>
    where
        T: Clone,
    {
        Octave(self.0.clone())
    }
    /// returns a new instance of the [`Octave`] with a copied instance of the current value
    pub const fn copied(&self) -> Octave<T>
    where
        T: Copy,
    {
        Octave(*self.0)
    }
}

impl<T> Octave<*const T>
{
    /// returns a new instance of the [`Octave`] with a copied instance of the current value
    pub const fn copied(&self) -> Octave<T>
    where
        T: Copy,
    {
        unsafe { Octave(*self.0) }
    }
}
