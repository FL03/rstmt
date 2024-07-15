/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Major, TriadKind};
use core::marker::PhantomData;
use rstmt::{Error, Third};

pub struct Triad<K = Major> {
    pub(crate) kind: PhantomData<K>,
    pub notes: [u8; 3],
}

impl<K> Triad<K>
where
    K: TriadKind,
{
    pub fn from_slice(notes: [u8; 3]) -> Result<Self, Error> {
        let (rt, tf) = K::thirds();
        let (r, t, f) = (notes[0], notes[1], notes[2]);
        let rt2 = Third::try_from(t - r);
        let tf2 = Third::try_from(f - t);
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
