/*
    appellation: pitch_class <module>
    authors: @FL03
*/
/// The [`RawPitchClass`] trait establishes an interface to defining pitch classes.
pub trait RawPitchClass: 'static + Send + Sync + core::fmt::Debug + core::fmt::Display {
    private!();
}
/*
 ************* Implementations *************
*/
macro_rules! class_enum {
    {
        $(#[doc $($doc:tt)*])?
        $vis:vis enum $name:ident {$($rest:tt)*}
    } => {
        $(#[doc $($doc)*])?
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
            scsys::VariantConstructors,
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
    };
}

macro_rules! pitch_class {
    {
        $($(#[doc $($doc:tt)*])?
        $vis:vis $i:ident $name:ident = $c:literal);* $(;)?
    } => {
        $(
            pitch_class! {
                @impl
                $(#[doc $($doc)*])?
                $vis $i $name
            }
            pitch_class!(@ext $name = $c);
        )*
    };
    {
        @impl
        $(#[doc $($doc:tt)*])?
        $vis:vis struct $name:ident
    } => {
        $(#[doc $($doc)*])?
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Deserialize, serde::Serialize),
        )]
        $vis struct $name;
    };
    (@ext $name:ident = $c:literal) => {
        impl $name {
            pub const C_MAJOR_ID: usize = $c;

            pub const fn value(self) -> usize {
                Self::C_MAJOR_ID
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }

        impl RawPitchClass for $name {
            seal!();
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

class_enum! {
    #[doc = "A representation of the natural pitch class"]
    pub enum Natural {
        #[default]
        C = 0,
        D = 2,
        E = 4,
        F = 5,
        G = 7,
        A = 9,
        B = 11,
    }
}

class_enum! {
    #[doc = "A representation of the sharp pitch class"]
    pub enum Sharp {
        #[default]
        C = 1,
        D = 3,
        F = 6,
        G = 8,
        A = 10,
    }
}

class_enum! {
    #[doc = "A representation of the flat pitch class"]
    pub enum Flat {
        #[default]
        D = 1,
        E = 3,
        G = 6,
        A = 8,
        B = 10,
    }
}

pitch_class! {
    #[doc = "A representation of the C pitch class"]
    pub struct C = 0;
    #[doc = "A representation of the D pitch class"]
    pub struct D = 2;
    #[doc = "A representation of the E pitch class"]
    pub struct E = 4;
    #[doc = "A representation of the F pitch class"]
    pub struct F = 5;
    #[doc = "A representation of the G pitch class"]
    pub struct G = 7;
    #[doc = "A representation of the A pitch class"]
    pub struct A = 9;
    #[doc = "A representation of the B pitch class"]
    pub struct B = 11;

    pub struct CSharp = 1;
    pub struct DSharp = 3;
    pub struct FSharp = 6;
    pub struct GSharp = 8;
    pub struct ASharp = 10;
    pub struct DFlat = 1;
    pub struct EFlat = 3;
    pub struct GFlat = 6;
    pub struct AFlat = 8;
    pub struct BFlat = 10;

}
