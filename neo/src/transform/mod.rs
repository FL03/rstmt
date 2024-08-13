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

pub trait Link<T> {
    type Dest;
}

/// [Apply] is an overloadable operator enabling implementations to define how
/// they should react to certain transformations.
pub trait Apply<T> {
    type Output;

    fn apply(self, apply: T) -> Self::Output;
}

pub(crate) mod utils {
    use super::LPR;
    use crate::prelude::TriadError;
    use crate::triad::*;
    use rstmt::{IntervalOps, Note, Third};

    fn _leading((r, t, f): (Note, Note, Note), rt: Third) -> (Note, Note, Note) {
        match rt {
            Third::Major => (t, f, r.sub_semitone()),
            Third::Minor => (f.add_semitone(), r, t),
        }
    }

    fn _parallel((r, t, f): (Note, Note, Note), rt: Third) -> (Note, Note, Note) {
        match rt {
            Third::Major => (r, t.sub_semitone(), f),
            Third::Minor => (r, t.add_semitone(), f),
        }
    }

    fn _relative((r, t, f): (Note, Note, Note), rt: Third) -> (Note, Note, Note) {
        match rt {
            Third::Major => (f.add_tone(), r, t),
            Third::Minor => (t, f, r.sub_tone()),
        }
    }
    pub(crate) fn _transform<K>(triad: Triad<K>, lpr: LPR) -> Result<Triad<K::Rel>, TriadError>
    where
        K: TriadKind,
    {
        let rt = triad.root_to_third()?;
        let chord = triad.into_tuple();
        let (r, t, f) = match lpr {
            LPR::L => _leading(chord, rt),
            LPR::P => _parallel(chord, rt),
            LPR::R => _relative(chord, rt),
        };

        Triad::try_from_notes(r, t, f)
    }
}
