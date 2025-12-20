/*
    Appellation: impl_pitch_repr <module>
    Created At: 2025.12.20:09:08:56
    Contrib: @FL03
*/
use super::Pitch;

impl<T> Pitch<&T> {
    /// returns a new instance of the [`Pitch`] containing a cloned inner value
    pub fn cloned(&self) -> Pitch<T>
    where
        T: Clone,
    {
        Pitch::new(self.0.clone())
    }
    /// returns a new instance of the [`Pitch`] containing a copied inner value
    pub const fn copied(&self) -> Pitch<T>
    where
        T: Copy,
    {
        Pitch::new(*self.0)
    }
}

impl<T> Pitch<&mut T> {
    /// returns a new instance of the [`Pitch`] containing a cloned inner value
    pub fn cloned(&self) -> Pitch<T>
    where
        T: Clone,
    {
        Pitch::new(self.0.clone())
    }
    /// returns a new instance of the [`Pitch`] containing a copied inner value
    pub const fn copied(&self) -> Pitch<T>
    where
        T: Copy,
    {
        Pitch::new(*self.0)
    }
}
