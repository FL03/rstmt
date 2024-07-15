/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Major, TriadKind};
use core::marker::PhantomData;
use rstmt::{Error, Note, Third};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]

pub struct Triad<K = Major> {
    pub(crate) kind: PhantomData<K>,
    pub notes: [Note; 3],
}

impl<K> Triad<K>
where
    K: TriadKind,
{
    pub fn new(root: Note) -> Self {
        let (rt, tf) = K::thirds();
        let octave = root.octave();
        let t = Note::new(root.pitch() + rt.value()).with_octave(octave);
        let f = Note::new(t.pitch() + tf.value()).with_octave(octave);
        Self {
            kind: PhantomData::<K>,
            notes: [root, t, f],
        }
    }
    pub fn from_slice(notes: [Note; 3]) -> Result<Self, Error> {
        let (rt, tf) = K::thirds();
        let (r, t, f) = (notes[0], notes[1], notes[2]);
        let rt2 = Third::try_from(*(t - r).pitch());
        let tf2 = Third::try_from(*(f - t).pitch());
        if rt2 == Ok(rt) && tf2 == Ok(tf) {
            Ok(Self {
                kind: PhantomData::<K>,
                notes,
            })
        } else {
            Err(Error::invalid_interval("Invalid interval."))
        }
    }
}
