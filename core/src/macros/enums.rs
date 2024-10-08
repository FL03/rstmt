/*
    Appellation: enums <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

macro_rules! unit_enum {
    ($(#[derive($($der:ident),*$(,)?)])? $vis:vis enum $class:ident $($rest:tt)*) => {
        unit_enum!(
            rename: "lowercase";
            $(#[derive($($der),*)])?
            $vis enum $class $($rest)*
        );
    };
    (rename: $rename:literal; $(#[derive($($der:ident),*$(,)?)])?  $vis:vis enum $class:ident $($rest:tt)*) => {

        #[derive(
            Clone,
            Copy,
            Debug,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd,
            strum::AsRefStr,
            strum::Display,
            strum::EnumCount,
            strum::EnumIs,
            strum::VariantNames,
            $($($der),*)?
        )]
        #[repr(i8)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize), serde(rename_all = $rename))]
        #[strum(serialize_all = $rename)]
        $vis enum $class $($rest)*
    };
}
