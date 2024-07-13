/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub type Tuple3<A = f64, B = A, C = B> = (A, B, C);

pub struct Triad {
    pub kind: super::Triads,
    pub notes: [u8; 3],
}
