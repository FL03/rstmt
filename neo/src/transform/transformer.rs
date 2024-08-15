/*
    Appellation: transformer <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{utils::*, LPR};
use crate::triad::{Triad, TriadKind, Triads};
use crate::TriadError;
use rstmt::prelude::{IntervalOps, Note, Third};

pub struct Transformer<'a, K> {
    delta: LPR,
    triad: &'a Triad<K>,
}

impl<'a, K> Transformer<'a, K> {
    pub fn new(triad: &'a Triad<K>, delta: LPR) -> Self {
        Self { delta, triad }
    }

    pub fn class(&self) -> Triads
    where
        K: 'static,
    {
        self.triad.class()
    }

    pub fn triad(&self) -> &Triad<K> {
        &self.triad
    }

    pub fn transform(self) -> Result<Triad<K::Rel>, TriadError>
    where
        K: TriadKind,
    {
        let rt = self.triad().root_to_third()?;
        let chord = self.triad().as_tuple();
        let (r, t, f) = match self.delta {
            LPR::L => _leading(chord, rt),
            LPR::P => _parallel(chord, rt),
            LPR::R => _relative(chord, rt),
        };

        Triad::try_from_notes(r, t, f)
    }
}
