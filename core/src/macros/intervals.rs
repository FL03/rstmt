/*
    Appellation: intervals <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

macro_rules! interval {
    ($vis:vis enum $name:ident {$($key:ident = $val:literal),* $(,)?}) => {
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
            strum::EnumIter,
            strum::EnumString,
            strum::VariantArray,
            strum::VariantNames,
        )]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize), serde(rename_all = "lowercase"))]
        #[repr(u8)]
        #[strum(serialize_all = "lowercase")]
        $vis enum $name {
            $($key = $val),*
        }

        impl From<$name> for u8 {
            fn from(interval: $name) -> u8 {
                interval as u8
            }
        }

        impl From<u8> for $name {
            fn from(interval: u8) -> $name {
                use strum::EnumCount;
                match interval % Self::COUNT as u8 {
                    $($val => $name::$key),*,
                    _ => panic!("Invalid interval value: {}", interval),
                }
            }
        }
    };
}
