/*
    Appellation: symbols <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// [Symbolic] is used to describe a type that has some
/// symbolic representation.
/// 
/// 
pub trait Symbolic {
    fn symbol(&self) -> &str;
}


