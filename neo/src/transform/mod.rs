/*
    Appellation: transform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::lpr::LPR;

pub(crate) mod lpr;

pub(crate) mod prelude {
    pub use super::lpr::LPR;
}

/// [Transform] describes objects capable of transformation. Specifically,
/// the [transform](Transform::transform) method is an overloadable operator
/// that enables implementations to define how they react to transformation(s).
pub trait Transform<T> {
    type Output;

    fn transform(self, apply: T) -> Self::Output;
}
