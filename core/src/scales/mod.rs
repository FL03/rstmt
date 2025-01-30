/*
    Appellation: scales <module>
    Contrib: @FL03
*/

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScaleNode {
    pub index: usize, // the position of the class within the whole
    pub symbol: String, // the symbolic representation
    pub weight: i8, // the modular weight of the pitch class (0-11)
}
