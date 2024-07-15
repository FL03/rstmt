/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Major, TriadKind};
use core::marker::PhantomData;

pub struct Triad<K = Major> {
    pub(crate) kind: PhantomData<K>,
    pub notes: [u8; 3],
}

impl<K> Triad<K>
where
    K: TriadKind,
{
    pub fn from_slice(notes: [u8; 3]) -> Self {
        Self {
            kind: PhantomData::<K>,
            notes,
        }
    }
}
