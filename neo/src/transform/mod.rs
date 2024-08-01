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
    use crate::prelude::TriadError;
    use crate::triad::*;
    use rstmt::Third;

    ///
    ///
    /// should result in a [Minor](crate::triad::Minor) triad.
    pub fn _leading<K>(triad: Triad<K>) -> Result<Triad<K::Rel>, TriadError>
    where
        K: TriadKind,
    {
        use rstmt::Intervals::Semitone;
        let rt = triad.root_to_third()?;
        let (mut r, t, mut f) = triad.into_tuple();
        match rt {
            Third::Major => {
                r -= Semitone;
                Triad::try_from_notes(t, f, r)
            }
            Third::Minor => {
                f += Semitone;
                Triad::try_from_notes(f, r, t)
            }
        }
    }

    pub fn _parallel<K>(triad: Triad<K>) -> Result<Triad<K::Rel>, TriadError>
    where
        K: TriadKind,
    {
        use rstmt::Intervals::Semitone;
        let rt = triad.root_to_third()?;
        let (r, mut t, mut f) = triad.into_tuple();
        match rt {
            Third::Major => {
                t -= Semitone;
                Triad::try_from_notes(f, r, t)
            }
            Third::Minor => {
                f += Semitone;
                Triad::try_from_notes(t, f, r)
            }
        }
    }

    pub fn _relative<K>(triad: Triad<K>) -> Result<Triad<K::Rel>, TriadError>
    where
        K: TriadKind,
    {
        use rstmt::Intervals::Tone;
        let rt = triad.root_to_third()?;
        let (mut r, t, mut f) = triad.into_tuple();
        match rt {
            Third::Major => {
                f += Tone;
                Triad::try_from_notes(f, r, t)
            }
            Third::Minor => {
                r -= Tone;
                Triad::try_from_notes(t, f, r)
            }
        }
    }

    pub(crate) fn _transform<K>(triad: Triad<K>, lpr: LPR) -> Result<Triad<K::Rel>, TriadError>
    where
        K: TriadKind,
    {
        match lpr {
            LPR::L => _leading(triad),
            LPR::P => _parallel(triad),
            LPR::R => _relative(triad),
        }
    }
}
