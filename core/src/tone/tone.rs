/*
    Appellation: tone <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::Pitch;

pub struct Tone<Q, S> {
    pitch: Pitch,
    quality: Q,
    strength: S,
}
