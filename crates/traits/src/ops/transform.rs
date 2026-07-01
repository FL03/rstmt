/*
    Appellation: transform <module>
    Created At: 2025.12.24:14:22:11
    Contrib: @FL03
*/

/// [`Dirac`] is used to define a specific transformation operation
pub trait Dirac<Rhs> {
    type Output;

    fn apply(self, rhs: Rhs) -> Self::Output;
}
/// The [`Transform`] trait establishes a binary operation that for objects capable of being
/// transformed by another object of type `Rhs`, producing some output.
///
pub trait Transform<Rhs> {
    type Output;

    fn transform(self, rhs: Rhs) -> Self::Output;
}
