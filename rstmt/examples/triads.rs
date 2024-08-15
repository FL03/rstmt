/*
    Appellation: triads <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use rstmt::neo::triad::*;
use rstmt::Note;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = Note::from_pitch(0);
    // initialize a c-major triad
    let triad = dbg!(Triad::major(root));
    assert_eq!(triad.root(), root);
    // test the parallel transformation
    assert_eq!(triad.parallel(), Triad::minor(root));
    // assert the invertibility of the transformations
    assert_eq!(triad.parallel().parallel(), triad);
    Ok(())
}

