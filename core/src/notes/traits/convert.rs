/*
    appellation: convert <module>
    authors: @FL03
*/
use crate::notes::Aspn;

/// The [`AsAspn`] trait is used to convert a reference into a [`Aspn`]
pub trait AsAspn {
    fn as_aspn(&self) -> Aspn;
}
/// A trait for converting a type into a [`Aspn`]
pub trait IntoAspn {
    fn into_aspn(self) -> Aspn;
}

/*
 ************* Implementations *************
*/
impl<T> AsAspn for T
where
    T: Clone + IntoAspn,
{
    fn as_aspn(&self) -> Aspn {
        self.clone().into_aspn()
    }
}

impl<T> IntoAspn for T
where
    T: Into<Aspn>,
{
    fn into_aspn(self) -> Aspn {
        self.into()
    }
}
