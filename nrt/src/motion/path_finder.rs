/*
    Appellation: path_finder <module>
    Created At: 2026.01.07:10:02:09
    Contrib: @FL03
*/
use super::config::PathFinderConfig;
use crate::traits::{TriadRepr, TriadType};
use crate::triad::TriadBase;
use rspace_traits::RawSpace;

/// The [`PathFinder`] implementation focuses on efficiently discovering transformation chains
/// capable of mutating a given traid into one containing a particular note or pitch class.
#[derive(Debug)]
pub struct PathFinder<'a, S, K, T = <S as RawSpace>::Elem>
where
    K: TriadType,
    S: TriadRepr<Elem = T>,
{
    pub(crate) config: PathFinderConfig,
    pub(crate) triad: &'a TriadBase<S, K, T>,
}
