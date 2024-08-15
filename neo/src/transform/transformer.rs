/*
    Appellation: transformer <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{utils::*, LPR};
use crate::triad::{Triad, TriadKind};
use crate::TriadError;

pub struct Transformer<K> {
    delta: Option<LPR>,
    triad: Triad<K>,
}

impl<K> Transformer<K> {
    pub(crate) fn new(triad: Triad<K>) -> Self {
        Self { delta: None, triad }
    }

    pub fn apply(self, delta: LPR) -> Self {
        Self {
            delta: Some(delta),
            ..self
        }
    }

    pub fn transform<J>(self) -> Result<Triad<J>, TriadError>
    where
        K: TriadKind<Rel = J>,
    {
        let Transformer { triad, delta } = self;
        let delta = delta.ok_or(TriadError::transformation_error(
            "No transformation specified...",
        ))?;
        let rt = triad.root_to_third()?;
        let chord = triad.as_tuple();
        let (r, t, f) = match delta {
            LPR::L => _leading(chord, rt),
            LPR::P => _parallel(chord, rt),
            LPR::R => _relative(chord, rt),
        };

        Triad::try_from_notes(r, t, f)
    }
}
