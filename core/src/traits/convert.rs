/*
    Appellation: convert <module>
    Contrib: @FL03
*/
use crate::Octave;


/// A trait for converting a reference into an [`Octave`].
pub trait AsOctave {
    fn as_octave(&self) -> Octave;
}
/// A trait for converting a type into an [`Octave`].
pub trait IntoOctave {
    fn into_octave(self) -> Octave;
}


/*
 ************* Implementations *************
*/

impl<T> AsOctave for T
where
    T: Clone + IntoOctave,
{
    fn as_octave(&self) -> Octave {
        self.clone().into_octave()
    }
}

impl<T> IntoOctave for T
where
    T: Into<Octave>,
{
    fn into_octave(self) -> Octave {
        self.into()
    }
}

