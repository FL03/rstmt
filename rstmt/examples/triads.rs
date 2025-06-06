/*
    Appellation: triads <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use rstmt::IntoNote;
use rstmt::nrt::Triad;

fn main() -> Result<(), Box<dyn core::error::Error + Send + Sync + 'static>> {
    let root = 0.into_note();
    // initialize a c-major triad
    let triad = dbg!(Triad::major(0));
    // test the root of the triad
    assert_eq!(triad.root(), root);
    // test the parallel transformation
    assert_eq!(triad.parallel(), Triad::minor(root));
    // assert the invertibility of the transformations
    assert_eq!(triad.parallel().parallel(), triad);

    Ok(())
}
