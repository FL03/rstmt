/*
    appellation: pitch_class <module>
    authors: @FL03
*/

macro_rules! classes {
    (@impl $name:ident) => {
        paste::paste! {
            pub type $name = PitchClass<[<$name Note>], Natural>;
        }
    };
    (@impl $name:ident::<$kind:ident>) => {
        paste::paste! {
            pub type [<$name $kind>] = PitchClass<[<$name $kind Note>], $kind>;
        }
    };
    ($($name:ident::<$($K:ident),* $(,)?>),* $(,)?) => {
            $(
                classes! { @impl $name }
                $(classes! { @impl $name::<$K> })*
            )*

    };
}

classes! { C::<Sharp>, D::<Flat, Sharp>, E::<Flat>, F::<Sharp>, G::<Flat, Sharp>, A::<Flat, Sharp>, B::<Flat>, }

/// The [`PitchClassifier`] trait establishes an interface to defining pitch classes.
pub trait PitchClassifier:
    'static + Send + Sized + Sync + core::fmt::Debug + core::fmt::Display
{
    private! {}

    fn new() -> Self;

    fn index(&self) -> usize;
}
/// [`PitchType`] is a sealed marker trait used to designate various _kinds_ of musical notes,
/// i.e., sharp, flat, natural, etc.
pub trait PitchType {
    private! {}
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[repr(C)]
pub struct PitchClass<N = CNote, K = Natural>
where
    K: PitchType,
{
    pub(crate) class: N,
    pub(crate) _marker: core::marker::PhantomData<K>,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[repr(transparent)]
pub struct ConstClass<const N: usize>;

/*
 ************* Implementations *************
*/
macro_rules! pitch_type {
    (@base $name:ident) => {
        impl PitchType for $name {
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

        pitch_type! { @base $name }
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

        pitch_type! { @base $name }
    };
    ($($vis:vis $type:ident $name:tt);* $(;)?) => {
        $(pitch_type! {
            @impl $vis $type $name;
        })*
    };
}

pitch_type! {
    pub struct Flat;
    pub struct Sharp;
    pub struct Natural;
}

macro_rules! class_enum {
    ($(#[doc $($doc:tt)*])? $vis:vis enum $name:ident {$($rest:tt)*}) => {
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
    };
}

macro_rules! pitch_class {
    {$($(#[$meta:meta])? $vis:vis $i:ident $name:ident = $c:literal);* $(;)?} => {
        $(
            pitch_class! {
                @impl
                $(#[$meta])?
                $vis $i $name
            }
            pitch_class! { @ext $name = $c }
        )*
    };
    {@impl $(#[$meta:meta])? $vis:vis struct $name:ident} => {
        $(#[$meta])?
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Deserialize, serde::Serialize),
            serde(rename_all = "UPPERCASE")
        )]
        #[repr(transparent)]
        $vis struct $name;
    };
    (@ext $name:ident = $c:literal) => {
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

        impl PitchClassifier for $name {
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

class_enum! {
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
    }
}

class_enum! {
    #[doc = "A representation of the sharp pitch class"]
    pub enum Sharps {
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
    pub enum Flats {
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
    pub struct CNote = 0;
    #[doc = "A representation of the D pitch class"]
    pub struct DNote = 2;
    #[doc = "A representation of the E pitch class"]
    pub struct ENote = 4;
    #[doc = "A representation of the F pitch class"]
    pub struct FNote = 5;
    #[doc = "A representation of the G pitch class"]
    pub struct GNote = 7;
    #[doc = "A representation of the A pitch class"]
    pub struct ANote = 9;
    #[doc = "A representation of the B pitch class"]
    pub struct BNote = 11;

    pub struct CSharpNote = 1;
    pub struct DSharpNote = 3;
    pub struct FSharpNote = 6;
    pub struct GSharpNote = 8;
    pub struct ASharpNote = 10;
    pub struct DFlatNote = 1;
    pub struct EFlatNote = 3;
    pub struct GFlatNote = 6;
    pub struct AFlatNote = 8;
    pub struct BFlatNote = 10;
}
