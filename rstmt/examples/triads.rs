/*
    Appellation: triads <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use rstmt::neo::triad::*;
use rstmt::Note;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = Note::from_pitch(0);
    let triad = Triad::major(root);
    println!("{triad:?}");
    Ok(())
}
