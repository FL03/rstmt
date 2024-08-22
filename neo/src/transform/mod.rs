/*
    Appellation: transform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{lpr::LPR, transformer::Transformer};

pub(crate) mod lpr;
pub(crate) mod transformer;

pub(crate) mod prelude {
    pub use super::lpr::LPR;
}

#[doc(hidden)]
/// [Transform] is an overloadable operator for objects capable of being transformed by the
/// given type.
pub trait Transform<F> {
    type Output;

    fn transform(self, apply: F) -> Self::Output;
}
pub(crate) mod utils {
    use super::LPR;
    use crate::prelude::NeoError;
    use rstmt::{IntervalOps, Note, Third};

    pub(crate) fn _transform(
        chord: (Note, Note, Note),
        dirac: LPR,
    ) -> Result<(Note, Note, Note), NeoError> {
        let rt = Third::new(chord.0, chord.1)?;
        let (r, t, f) = match dirac {
            LPR::L => _leading(chord, rt),
            LPR::P => _parallel(chord, rt),
            LPR::R => _relative(chord, rt),
        };

        Ok((r, t, f))
    }

    pub(crate) fn _leading((r, t, f): (Note, Note, Note), rt: Third) -> (Note, Note, Note) {
        match rt {
            Third::Major => (t, f, r.sub_semitone()),
            Third::Minor => (f.add_semitone(), r, t),
        }
    }

    pub(crate) fn _parallel((r, t, f): (Note, Note, Note), rt: Third) -> (Note, Note, Note) {
        match rt {
            Third::Major => (r, t.sub_semitone(), f),
            Third::Minor => (r, t.add_semitone(), f),
        }
    }

    pub(crate) fn _relative((r, t, f): (Note, Note, Note), rt: Third) -> (Note, Note, Note) {
        match rt {
            Third::Major => (f.add_tone(), r, t),
            Third::Minor => (t, f, r.sub_tone()),
        }
    }
}
