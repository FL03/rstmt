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

#[cfg(test)]
mod tests {
    use crate::Triad;

    #[test]
    fn test_triad_path_finder() -> crate::Result<()> {
        // set the target note to 1 (C#)
        let tgt: isize = 1;
        // initialize a new, c-major triad: [0, 4, 7]
        let c_major = Triad::major(0).dynamic();
        // use the transformer to find all paths between the current instance the target note
        let paths = c_major.path_finder().find_paths_to_target(tgt)?;
        // verify each path results in an instance containing the target note.
        for chain in paths {
            let p = chain.path().iter().copied();
            assert! { c_major.walk(p).contains(&tgt) }
        }
        Ok(())
    }

    #[test]
    #[cfg(feature = "rand")]
    fn test_triad_path_finder_rand() -> crate::Result<()> {
        let root: usize = rand::random_range(0..12);
        let rtgt: usize = rand::random_range(1..12);
        let target = rtgt as isize;
        // initialize a random major triad
        let triad = Triad::major(root as isize).dynamic();
        // use the transformer to find all paths between the current instance the target note
        let paths = triad.path_finder().find_paths_to_target(target)?;
        // verify that all of the paths result in an instance containing the target note.
        for chain in paths {
            let p = chain.path().clone();
            assert! { triad.walk(p).contains(&target) }
        }
        Ok(())
    }
}
