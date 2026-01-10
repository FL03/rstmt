/*
    Appellation: triads <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use rstmt::nrt::Triad;

fn main() -> anyhow::Result<()> {
    // Set up tracing
    tracing_subscriber::fmt()
        .with_line_number(false)
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .init();
    // initialize a c-major triad
    let triad = Triad::major(0);
    tracing::debug! { "Initialized triad: {:?}", triad }
    // verify the triads composition and quality
    assert_eq! { triad, [0, 4, 7] }
    assert! { triad.is_major() }
    // test the parallel transformation and verify that it is its own inverse
    let p_triad = triad.parallel();
    tracing::debug! { "Parallel triad: {:?}", p_triad }
    assert_eq! { p_triad, [0, 3, 7] }
    assert_eq! { p_triad.parallel(), triad }

    let target = 6;
    let dyntriad = triad.dynamic();
    let paths = dyntriad.path_finder().find_paths_to_target(target)?;
    tracing::info! {
        "Found {} paths from C Major to triads containing {}:",
        paths.len(), target
    }
    for (i, chain) in paths.iter().enumerate() {
        tracing::info!("Path {}: {:?}", i + 1, chain.path());
        let res = dyntriad.walk(chain.path().iter().copied());
        tracing::info! { "Resulting triad: {:?}", res }
        assert! { res.contains(&target), "the resulting triad must contain the target ({target}) note" }
    }

    Ok(())
}
