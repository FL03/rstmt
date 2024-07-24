/*
    Appellation: triads <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use rstmt::neo::triad::*;
use rstmt::Note;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = Note::from_pitch(0);
    let triad = Triad::<Major>::from_root(root);
    let triad2 = Triad::new(Triads::Major, root).cast();
    assert_eq!(triad, triad2);
    println!("{:?}", triad);
    Ok(())
}
