/*
    Appellation: navigator <test>
    Contrib: @FL03
*/
use rstmt_nrt::Triad;

#[test]
fn test_triad_path_finder() -> anyhow::Result<()> {
    let triad = Triad::major(0).dynamic(); // C Major (0,4,7)
    // set the target note to 1 (C#)
    let target_note: isize = 1;
    // use the transformer to find all paths between the current instance the target note
    let paths = triad.path_finder().find_paths_to_target(target_note)?;
    // verify each path results in an instance containing the target note.
    paths.iter().for_each(|chain| {
        let p = chain.path().clone();
        assert! { triad.walk(p).contains(&target_note) }
    });
    Ok(())
}

#[cfg(feature = "rand")]
#[test]
fn test_triad_path_finder_rand() -> anyhow::Result<()> {
    let rand_root: usize = rand::random_range(0..12);
    let rand_tgt: usize = rand::random_range(0..12);
    let target_note = rand_tgt as isize;
    // initialize a random major triad
    let triad = Triad::major(rand_root as isize).dynamic();
    // use the transformer to find all paths between the current instance the target note
    let paths = triad.path_finder().find_paths_to_target(target_note)?;
    // verify that all of the paths result in an instance containing the target note.
    for chain in paths {
        let p = chain.path().clone();
        assert! { triad.walk(p).contains(&target_note) }
    }
    Ok(())
}
