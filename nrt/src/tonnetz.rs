/*
    Appellation: tonnetz <module>
    Created At: 2025.12.28:10:54:06
    Contrib: @FL03
*/
//! The tonnetz is a conceptual lattice representation of tonal space first proposed in 1739 by
//! Leonhard Euler as a means of visualizing relationships between triads
//!
//! ## Resources
//!
//! Listed below are some useful resources for understanding the tonnetz and its potential
//! applications in both music theroy as well as computer science:
//!
//! - [The Generalized Tonnetz](https://dmitri.mycpanel.princeton.edu/tonnetzes.pdf)
//! - [Wikipedia: Tonnetz](https://en.wikipedia.org/wiki/Tonnetz)
//!
mod impl_hyper_tonnetz;

use crate::triad::TriadBase;
use crate::{LPR, TriadRepr, TriadType, Triads};
use hashbrown::HashMap;
use rshyper::{EdgeId, HyperMap};
use rspace_traits::RawSpace;

/// a type alias for a [`HashMap`] that maps an [`EdgeId`] to a [`TriadBase`]
pub(crate) type TriadMap<S = [usize; 3], K = Triads, T = <S as RawSpace>::Elem, Ix = usize> =
    HashMap<EdgeId<Ix>, TriadBase<S, K, T>>;
/// a type alias for a [`HashMap`] that maps an [`EdgeId`] to a [`HashMap`] of [`LPR`]
/// transformations.
pub(crate) type LprMap<I = usize> = HashMap<EdgeId<I>, HashMap<LPR, EdgeId<I>>>;

/// The [`HyperTonnetz`] implementation relies on a _hypergraph_ to define the relationships
/// between various notes and triads within the tonal space. Hypergraphs generalize the concept
/// of a graph by allowing edges to connect any number of vertices, making them well-suited
/// for modeling complex relationships and topologies such as those found in music theory.
#[derive(Clone, Debug)]
pub struct HyperTonnetz<S = [usize; 3], K = Triads, T = <S as RawSpace>::Elem, Ix = usize>
where
    K: TriadType,
    S: TriadRepr<Elem = T>,
{
    /// a hypergraph representing the tonal space
    pub(crate) graph: HyperMap<T>,
    /// Maps EdgeIds to Triad for efficient access
    pub(crate) triads: TriadMap<S, K, T, Ix>,
    /// Tracks adjacency between triads via transformations
    pub(crate) transformations: LprMap<Ix>,
}
