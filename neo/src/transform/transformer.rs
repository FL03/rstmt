/*
    Appellation: transformer <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{utils::*, LPR};
use crate::triad::{Triad, TriadKind};
use crate::NeoError;

pub struct Transformer<K> {
    delta: LPR,
    triad: Triad<K>,
}

impl<K> Transformer<K> {
    #[allow(dead_code)]
    pub(crate) fn new(delta: LPR, triad: Triad<K>) -> Self {
        Self { delta, triad }
    }

    pub fn from_triad(triad: Triad<K>) -> Self {
        Self {
            delta: LPR::L,
            triad,
        }
    }

    pub fn with_transform(self, delta: LPR) -> Self {
        Self { delta, ..self }
    }

    pub fn leading(self) -> Self {
        self.with_transform(LPR::L)
    }

    pub fn parallel(self) -> Self {
        self.with_transform(LPR::P)
    }

    pub fn relative(self) -> Self {
        self.with_transform(LPR::R)
    }
    /// Transforms a triad into another triad based on the specified transformation.
    pub fn apply<J>(self) -> Result<Triad<J>, NeoError>
    where
        K: TriadKind<Rel = J>,
    {
        let Transformer { triad, delta } = self;
        // compute the interval between the root and third factors
        let rt = rstmt::Third::new(triad.root(), triad.third())?;
        // convert the triad into a 3-tuple
        let chord = triad.as_tuple();
        // match the transformation
        let (r, t, f) = match delta {
            LPR::L => _leading(chord, rt),
            LPR::P => _parallel(chord, rt),
            LPR::R => _relative(chord, rt),
        };
        // try constructing a new triad from the transformed notes
        Triad::try_from_notes(r, t, f)
    }
}
