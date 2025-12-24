/*
    Appellation: triads <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use rstmt::nrt::Triad;

fn main() -> anyhow::Result<()> {
    // initialize a c-major triad
    let triad = dbg!(Triad::major(0));
    // test the root of the triad
    assert_eq! { triad, [0, 4, 7] }
    assert! { triad.is_major() }
    // test the parallel transformation
    assert_eq!(triad.parallel(), [0, 3, 7]);
    // assert the invertibility of the transformations
    assert_eq!(triad.parallel().parallel(), triad);

    Ok(())
}
