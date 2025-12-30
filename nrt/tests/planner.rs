/*
    Appellation: motion <module>
    Contrib: @FL03
*/
use rstmt_core::Octave;
use rstmt_nrt::motion::MotionPlanner;
use rstmt_nrt::{HyperTonnetz, Triad, TriadError};

#[test]
fn test_motion_planner() -> Result<(), TriadError> {
    // Create a new tonnetz
    let mut tonnetz = HyperTonnetz::new();

    // Add pitch classes (0-11)
    let _ = tonnetz.scaffold_layer(Octave(4))?;

    let c_major = Triad::major(0); // C Major (0,4,7)
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
