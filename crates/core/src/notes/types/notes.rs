/*
    Appellation: class_enums <module>
    Created At: 2025.12.23:14:44:47
    Contrib: @FL03
*/
//! a more dynamic approach to managing pitch classes using enums

/// [`Notes`] is an enumeration of all allowed symbolic representations of pitch classes
/// considered in music theory. For us, it provides a dynamic way of managing the different
/// pitch classes whiole providing a direct mapping to a static index
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    smart_default::SmartDefault,
    strum::EnumCount,
    strum::EnumIs,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[strum(serialize_all = "lowercase")]
pub enum Notes {
    Sharp(Sharps),
    Flat(Flats),
    #[default]
    Natural(Naturals),
}

macro_rules! pitch_class_enums {
    ($($(#[$meta:meta])? $vis:vis enum $name:ident {$($rest:tt)*});* $(;)?) => {
        $(
            $(#[$meta])?
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
                serde(rename_all = "UPPERCASE")
            )]
            #[strum(serialize_all = "UPPERCASE")]
            $vis enum $name {$($rest)*}
        )*
    };
}

/*
 ************* Implementations *************
*/
pitch_class_enums! {
    #[doc = "A representation of the natural pitch class"]
    pub enum Naturals {
        #[default]
        C = 0,
        D = 2,
        E = 4,
        F = 5,
        G = 7,
        A = 9,
        B = 11,
    };
    #[doc = "A representation of the sharp pitch class"]
    pub enum Sharps {
        #[default]
        C = 1,
        D = 3,
        F = 6,
        G = 8,
        A = 10,
    };
    #[doc = "A representation of the flat pitch class"]
    pub enum Flats {
        #[default]
        D = 1,
        E = 3,
        G = 6,
        A = 8,
        B = 10,
    };
}
