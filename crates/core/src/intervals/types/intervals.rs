pub enum Intervals {
    Third(Thirds),
    Fourth(Fourths),
}

macro_rules! intervals {
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

intervals! {
    /// Represents the various kinds of thirds in music theory.
    pub enum Thirds {
        #[default]
        Major = 4,
        Minor = 3,
    };
    pub enum Fourths {
        #[default]
        Perfect = 5,
        Augmented = 6,
        Diminished = 4,
    };
    pub enum Fifths {
        #[default]
        Perfect = 7,
        Augmented = 8,
        Diminished = 6,
    };
    pub enum Sixths {
        #[default]
        Major = 10,
        Minor = 9,
    };
    pub enum Sevenths {
        #[default]
        Major = 11,
        Minor = 10,
        Perfect = 12,
        Diminished = 9,
    };
}
