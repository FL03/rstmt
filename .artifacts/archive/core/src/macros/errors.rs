/*
    Appellation: errors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

macro_rules! error {
    (@impl $name:ident {$($err:ident: $msg:literal),* $(,)?}) => {
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
            strum::EnumIs,
            strum::EnumString,
            strum::VariantNames,
        )]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Deserialize, serde::Serialize),
            serde(rename_all = "PascalCase")
        )]
        #[strum(serialize_all = "PascalCase")]
        pub enum $name {
            $($err),*
        }

        impl $name {

        }

        #[cfg(feature = "std")]
        impl std::error::Error for $name {}

        unsafe impl Send for $name {}

        unsafe impl Sync for $name {}
    };
}
