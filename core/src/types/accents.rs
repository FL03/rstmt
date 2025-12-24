/*
    Appellation: accents <module>
    Created At: 2025.12.23:15:57:18
    Contrib: @FL03
*/

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    strum::EnumCount,
    strum::EnumDiscriminants,
    strum::EnumIs,
)]
#[strum_discriminants(
    name(NoteFlag),
    derive(
        Hash,
        Ord,
        PartialOrd,
        strum::AsRefStr,
        strum::Display,
        strum::EnumCount,
        strum::EnumIs,
        strum::EnumIter,
        strum::EnumString,
        strum::VariantArray,
        strum::VariantNames,
    ),
    strum(serialize_all = "lowercase")
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase"),
    strum_discriminants(
        derive(serde::Deserialize, serde::Serialize),
        serde(rename_all = "lowercase")
    )
)]
pub enum SingleOrDouble<T> {
    /// A generic `single` state representing single sharp/flat notes
    Single(T),
    /// A generic `double` state representing double sharp/flat notes
    Double(T),
}

/*
 ************* Implementations *************
*/
impl<T> SingleOrDouble<T> {
    pub const fn single(value: T) -> Self {
        Self::Single(value)
    }
    pub const fn double(value: T) -> Self {
        Self::Double(value)
    }
    /// returns a reference to the inner value
    pub const fn get(&self) -> &T {
        match self {
            Self::Single(v) => v,
            Self::Double(v) => v,
        }
    }
    /// returns a mutable reference to the inner value
    pub const fn get_mut(&mut self) -> &mut T {
        match self {
            Self::Single(v) => v,
            Self::Double(v) => v,
        }
    }
}
