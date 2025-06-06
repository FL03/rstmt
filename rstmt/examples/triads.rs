/*
    Appellation: triads <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use rstmt::Note;
use rstmt::nrt::Triad;

fn main() -> Result<(), Box<dyn core::error::Error + Send + Sync + 'static>> {
    let root = Note::from_pitch(0);
    // initialize a c-major triad
    let triad = dbg!(Triad::major(root));
    // test the root of the triad
    assert_eq!(triad.root(), root);
    // test the parallel transformation
    assert_eq!(triad.parallel(), Triad::minor(root));
    // assert the invertibility of the transformations
    assert_eq!(triad.parallel().parallel(), triad);

    Ok(())
}
