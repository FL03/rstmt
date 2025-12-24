/*
    Appellation: octave <types>
    Contrib: @FL03
*/
mod impl_octave;
mod impl_octave_ext;
mod impl_octave_ops;
#[cfg(feature = "rand")]
mod impl_octave_rand;
mod impl_octave_repr;

/// A trait for converting a reference into an [`Octave`].
pub trait AsOctave<T> {
    fn as_octave(&self) -> Octave<T>;
}
/// A trait for converting a type into an [`Octave`].
pub trait IntoOctave<T> {
    fn into_octave(self) -> Octave<T>;
}

/// A type defining an octave
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
#[repr(transparent)]
pub struct Octave<T = isize>(pub T);

/*
 ************* Implementations *************
*/

impl<U, T> AsOctave<T> for U
where
    U: Clone + IntoOctave<T>,
{
    fn as_octave(&self) -> Octave<T> {
        self.clone().into_octave()
    }
}

impl<U, T> IntoOctave<T> for U
where
    U: Into<Octave<T>>,
{
    fn into_octave(self) -> Octave<T> {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_into_octave() {
        let octave: Octave = 4isize.into_octave();
        assert_eq!(octave, 4);

        let octave_ref: Octave = octave.as_octave();
        assert_eq!(octave_ref.0, 4);
    }
}
