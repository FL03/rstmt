/*
    Appellation: motion <planner>
    Contrib: @FL03
*/
use crate::motion::config::MotionPlannerConfig;
use crate::motion::types::PathCache;
use crate::tonnetz::StdHyperTonnetz;
use core::hash::Hash;

/// The [`MotionPlanner`] is a pathfinding algorithm implementation for finding the chain of
/// transformations between two triads along the surface of the hyper-tonnetz.
pub struct MotionPlanner<'a, T = usize>
where
    T: Eq + Hash,
{
    /// Cache for storing computed paths
    pub(crate) cache: PathCache<T>,
    /// Reference to the tonnetz graph
    pub(crate) tonnetz: &'a StdHyperTonnetz<T>,
    pub(crate) config: MotionPlannerConfig,
}

#[cfg(test)]
mod tests {
    use super::MotionPlanner;
    use crate::tonnetz::HyperTonnetz;
    use crate::triad::Triad;
    use rstmt_core::Octave;

    #[test]
    fn test_motion_planner() -> crate::Result<()> {
        // initialize an empty tonnetz
        let mut tonnetz = HyperTonnetz::new();
        // scaffold a layer for the 4th octave
        let _ = tonnetz.scaffold_layer(Octave(4))?;

        let c_major = Triad::major(0).dynamic(); // C Major (0,4,7)
        // Add some common triads to the Tonnetz
        let c_major_idx = tonnetz.add_triad(c_major)?; // C Major (0,4,7)

        // Create a motion planner
        let mut planner = MotionPlanner::new(&tonnetz).with_max_depth(4);

        // Find paths from C Major to triads containing C# (1)
        let target_note = 1; // C#
        let paths = planner.find_paths_to_pitch(c_major_idx, target_note);

        assert!(!paths.is_empty());
        for path in paths {
            assert!(path.triads().last().unwrap().contains(&target_note));
        }

        Ok(())
    }
}
