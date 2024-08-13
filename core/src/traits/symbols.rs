/*
    Appellation: symbols <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// [Symbolic] describes objects capable of symbolic representation
pub trait Symbolic {
    fn symbol(&self) -> &str;
}
