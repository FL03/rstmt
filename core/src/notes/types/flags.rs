/*
    appellation: flags <module>
    authors: @FL03
*/
/// [`NoteKind`] is a sealed marker trait used to designate various _kinds_ of musical notes, 
/// i.e., sharp, flat, natural, etc.
pub trait NoteKind {
    private! {}
}

/// [`Accidental`] enumerates the two alternative states a note make take, either sharp or flat
/// and their respective states, single or double.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    variants::VariantConstructors,
    strum::EnumCount,
    strum::EnumIs,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[strum(serialize_all = "lowercase")]
pub enum Accidental {
    Sharp(AccidentalState),
    Flat(AccidentalState),
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    variants::VariantConstructors,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::EnumIter,
    strum::EnumString,
    strum::VariantArray,
    strum::VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[strum(serialize_all = "lowercase")]
pub enum AccidentalState {
    #[default]
    /// A [`Single`](AccidentalState::Single) accidental state represents typical sharp/flat note
    Single,
    /// A [`Double`](AccidentalState::Double) accidental state represents a double sharp/flat note
    Double,
}

macro_rules! note_tag {
    (@base $name:ident) => {
        impl NoteKind for $name {
            seal! {}
        }
    };
    (@impl $vis:vis enum $name:ident $(;)?) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Deserialize, serde::Serialize),
            serde(rename_all = "lowercase")
        )]
        #[repr(transparent)]
        $vis enum $name {}

        note_tag! { @base $name }
    };
    (@impl $vis:vis struct $name:ident $(;)?) => {
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Deserialize, serde::Serialize),
            serde(rename_all = "lowercase")
        )]
        #[repr(transparent)]
        $vis struct $name;

        note_tag! { @base $name }
    };
    ($($vis:vis $type:ident $name:tt);* $(;)?) => {
        $(note_tag! { 
            @impl $vis $type $name; 
        })*
    };
}

note_tag! {
    pub struct Flat;
    pub struct Sharp;
    pub struct Natural;
}