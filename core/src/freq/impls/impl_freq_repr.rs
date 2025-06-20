/*
    appellation: impl_freq_repr <module>
    authors: @FL03
*/
use crate::freq::Frequency;

impl<T> Frequency<&T> {
    /// returns a new instance of the [`Frequency`] with a cloned instance of the current value.
    pub fn cloned(&self) -> Frequency<T>
    where
        T: Clone,
    {
        Frequency(self.0.clone())
    }

    /// returns a new instance of the [`Frequency`] with a copied instance of the current value.
    pub fn copied(&self) -> Frequency<T>
    where
        T: Copy,
    {
        Frequency(*self.0)
    }
}

impl<T> Frequency<&mut T> {
    /// returns a new instance of the [`Frequency`] with a cloned instance of the current value.
    pub fn cloned(&self) -> Frequency<T>
    where
        T: Clone,
    {
        Frequency(self.0.clone())
    }

    /// returns a new instance of the [`Frequency`] with a copied instance of the current value.
    pub fn copied(&self) -> Frequency<T>
    where
        T: Copy,
    {
        Frequency(*self.0)
    }
}
