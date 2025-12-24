/*
    Appellation: navigator <test>
    Contrib: @FL03
*/
use rstmt_nrt::Triad;

#[test]
fn test_transformer() -> anyhow::Result<()> {
    let triad = Triad::major(0); // C Major (0,4,7)
    // set the target note to 1 (C#)
    let target_note: usize = 1;
    // use the transformer to find all paths between the current instance the target note
    let paths = triad.path_finder().find_paths_to_target(target_note)?;
    // verify that all of the paths result in an instance containing the target note.
    for chain in paths {
        let p = chain.path().clone();
        assert! { triad.walk(p).contains(&target_note) }
    }
    Ok(())
}

#[cfg(feature = "rand")]
#[test]
fn test_transformer_rand() -> anyhow::Result<()> {
    let root: usize = rand::random_range(0..12);
    let triad = dbg!(Triad::major(root));

    // set the target note to 1 (C#)
    let target_note: usize = rand::random_range(0..12);
    // use the transformer to find all paths between the current instance the target note
    let paths = triad.path_finder().find_paths_to_target(target_note)?;
    // verify that all of the paths result in an instance containing the target note.
    for chain in paths {
        let p = chain.path().clone();
        assert! { triad.walk(p).contains(&target_note) }
    }
    Ok(())
}
