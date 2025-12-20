/*
    Appellation: classify <module>
    Created At: 2025.12.20:06:14:19
    Contrib: @FL03
*/
/// [`Classify`] defines an interface for objects capable of being classified as other types.
pub trait Classify {
    type Output;

    fn classify(&self) -> Self::Output;
}
/// [`ClassifyBy`] is a trait defining the ability for an object to be classified _by_ or _with_
/// another object.
pub trait ClassifyBy<Rhs> {
    type Output;

    fn classify_by(&self, rhs: Rhs) -> Self::Output;
}
