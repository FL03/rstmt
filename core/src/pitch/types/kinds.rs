/*
    Appellation: pitch_kinds <module>
    Created At: 2025.12.20:10:03:26
    Contrib: @FL03
*/

macro_rules! classes {
    (@impl $name:ident::<Natural>) => {
        paste::paste! {
            pub type $name = $crate::pitch::PitchClass<$crate::pitch::[<$name Note>], $crate::pitch::Natural>;
        }
    };
    (@impl $name:ident::<$kind:ident>) => {
        paste::paste! {
            pub type [<$name $kind>] = $crate::pitch::PitchClass<$crate::pitch::[<$name $kind Note>], $crate::pitch::$kind>;
        }
    };
    (@impl $name:ident::<$($kind:ident),+ $(,)?>) => {
        $(classes! { @impl $name::<$kind> })*
    };
    ($($name:ident::<$($K:ident),* $(,)?>),* $(,)?) => {
        $(classes! { @impl $name::<Natural, $($K),*> })*
    };
}

macro_rules! create_class_enums {
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

macro_rules! pitch_class {
    {$($(#[$meta:meta])* $vis:vis $i:ident $name:ident<$tag:ty> = $c:literal);* $(;)?} => {
        $(
            pitch_class! {@impl $(#[$meta])* $vis $i $name<$tag> }
            pitch_class! { @ext $name<$tag> = $c }
        )*
    };
    {@impl $(#[$meta:meta])* $vis:vis struct $name:ident<$tag:ty>} => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Deserialize, serde::Serialize),
            serde(rename_all = "UPPERCASE")
        )]
        #[repr(transparent)]
        $vis struct $name;
    };
    (@ext $name:ident<$tag:ty> = $c:literal) => {
        impl $name {
            pub const C_MAJOR_ID: usize = $c;
            /// returns a new instance of the pitch class
            pub const fn new() -> Self {
                Self
            }
            /// returns a copy of the assigned index on the c-major scale
            pub const fn value(&self) -> usize {
                Self::C_MAJOR_ID
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }

        impl $crate::pitch::PitchCls for $name {
            const IDX: usize = $c;
            type Tag = $tag;

            seal! {}

            fn new() -> Self {
                Self::new()
            }

            fn index(&self) -> usize {
                self.value()
            }
        }

        impl PartialEq<usize> for $name {
            fn eq(&self, other: &usize) -> bool {
                self.value() == *other
            }
        }

        impl PartialEq<$name> for usize {
            fn eq(&self, other: &$name) -> bool {
                *self == other.value()
            }
        }

        impl PartialOrd<usize> for $name {
            fn partial_cmp(&self, other: &usize) -> Option<core::cmp::Ordering> {
                Some(self.value().cmp(other))
            }
        }

        impl PartialOrd<$name> for usize {
            fn partial_cmp(&self, other: &$name) -> Option<core::cmp::Ordering> {
                Some(self.cmp(&other.value()))
            }
        }
    };
}

macro_rules! pitch_type {
    (@impl $(#[$meta:meta])* $vis:vis enum $name:ident $(;)?) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        $vis enum $name {}
    };
    (@impl $(#[$meta:meta])* $vis:vis struct $name:ident $(;)?) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        $vis struct $name;
    };
    ($($vis:vis $type:ident $name:ident);* $(;)?) => {
        $(
            pitch_type! { @impl
                #[cfg_attr(
                    feature = "serde",
                    derive(serde::Deserialize, serde::Serialize),
                    serde(rename_all = "lowercase")
                )]
                #[repr(transparent)]
                $vis $type $name
            }

            impl $crate::pitch::PitchType for $name {
                seal! {}
            }
        )*
    };
}

/*
 ************* Implementations *************
*/
create_class_enums! {
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

pitch_type! {
    pub struct Flat;
    pub struct Sharp;
    pub struct Natural;
}

pitch_class! {
    pub struct CNote<Natural> = 0;
    pub struct DNote<Natural> = 2;
    pub struct ENote<Natural> = 4;
    pub struct FNote<Natural> = 5;
    pub struct GNote<Natural> = 7;
    pub struct ANote<Natural> = 9;
    pub struct BNote<Natural> = 11;
    pub struct CSharpNote<Sharp> = 1;
    pub struct DSharpNote<Sharp> = 3;
    pub struct FSharpNote<Sharp> = 6;
    pub struct GSharpNote<Sharp> = 8;
    pub struct ASharpNote<Sharp> = 10;
    pub struct DFlatNote<Flat> = 1;
    pub struct EFlatNote<Flat> = 3;
    pub struct GFlatNote<Flat> = 6;
    pub struct AFlatNote<Flat> = 8;
    pub struct BFlatNote<Flat> = 10;
}

classes! {
    C::<Sharp>,
    D::<Flat, Sharp>,
    E::<Flat>,
    F::<Sharp>,
    G::<Flat, Sharp>,
    A::<Flat, Sharp>,
    B::<Flat>,
}
