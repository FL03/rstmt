/*
    Appellation: walker <module>
    Created At: 2025.12.24:15:37:32
    Contrib: @FL03
*/
#![allow(dead_code)]
use crate::types::LPR;
use crate::{RawTriad, TriadBase, TriadCls};
use rspace_traits::RawSpace;

pub struct Walk<'a, P, S, K, T = <S as RawSpace>::Elem>
where
    S: RawTriad<Elem = T>,
    K: TriadCls,
    P: Iterator<Item = LPR>,
{
    pub(crate) triad: &'a mut TriadBase<S, K, T>,
    pub(crate) path: P,
}

impl<'a, P, K, T> Walk<'a, P, [T; 3], K, T>
where
    K: TriadCls,
    P: Iterator<Item = LPR>,
{
    pub(crate) fn new(triad: &'a mut TriadBase<[T; 3], K, T>, path: P) -> Self {
        Self { triad, path }
    }

    pub fn walk(&mut self) -> &mut TriadBase<[T; 3], K, T> {
        // for step in self.path.by_ref() {
        //     self.triad.transform_inplace(step);
        // }
        // self.triad
        unimplemented! { "" }
    }
}
