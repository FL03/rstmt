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
//// [`ClassifyWith`] defines an interface for objects capable of being classified with respect 
/// to another object.
pub trait ClassifyWith<Rhs> {
    type Output;

    fn classify_with(&self, rhs: Rhs) -> Self::Output;
}