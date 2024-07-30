/*
    Appellation: transform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::lpr::LPR;
#[allow(unused)]
pub(crate) use self::utils::*;

pub(crate) mod lpr;

pub(crate) mod prelude {
    pub use super::lpr::LPR;
}

/// [Apply] is an overloadable operator enabling implementations to define how
/// they should react to certain transformations.
pub trait Apply<T> {
    type Output;

    fn apply(self, apply: T) -> Self::Output;
}

/// [Transform] describes objects capable of transformation. Specifically,
/// the [transform](Transform::transform) method is an overloadable operator
/// that enables implementations to define how they react to transformation(s).
pub trait Transform<T> {
    type Output;

    fn transform(self, apply: T) -> Self::Output;
}

impl<T, K> Apply<T> for K
where
    K: Transform<T>,
{
    type Output = K::Output;

    fn apply(self, apply: T) -> Self::Output {
        self.transform(apply)
    }
}

pub(crate) mod utils {
    use super::LPR;
    use crate::error::TriadError;
    use crate::triad::{Triad, TriadCls, Triads};
    use rstmt::Third;

    pub(crate) fn _transform<K>(triad: Triad<K>, lpr: LPR) -> Result<Triad<Triads>, TriadError>
    where
        K: Clone + TriadCls,
    {
        use rstmt::Intervals::{Semitone, Tone};
        let (mut r, mut t, mut f) = triad.clone().into_tuple();
        match triad.root_to_third()? {
            Third::Major => match lpr {
                LPR::L => r -= Semitone,
                LPR::P => t -= Semitone,
                LPR::R => f += Tone,
            },
            Third::Minor => match lpr {
                LPR::L => r += Semitone,
                LPR::P => t += Semitone,
                LPR::R => f -= Tone,
            },
        };

        Triad::try_from_arr([r, t, f])
    }
}
