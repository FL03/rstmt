/*
    Appellation: classes <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::TriadError;
use core::marker::PhantomData;
use rstmt::{Fifth, Note, Third};
use strum::IntoEnumIterator;

pub trait Named {
    fn named(&self) -> &'static str;
}

pub trait Relative {
    type Rel;

    fn relative(&self) -> Self::Rel;
}

/// This trait denotes privately declared instances of different classes of triads.
/// Traditionally, triads have two primary classes: [major](Major) and [minor](Minor), however, there are
/// two additional classes: [augmented](Augmented) and [diminished](Diminished). This trait is used to determine
pub trait TriadCls {
    private!();

    fn named(&self) -> &'static str;
}

impl TriadCls for Triads {
    seal!();

    fn named(&self) -> &'static str {
        match self {
            Triads::Augmented => "augmented",
            Triads::Diminished => "diminished",
            Triads::Major => "major",
            Triads::Minor => "minor",
        }
    }
}

impl<K> TriadCls for PhantomData<K>
where
    K: TriadKind,
{
    seal!();

    fn named(&self) -> &'static str {
        K::name()
    }
}

impl<K> Relative for PhantomData<K>
where
    K: TriadKind,
{
    type Rel = Triads;

    fn relative(&self) -> Self::Rel {
        K::class()
    }
}

impl<K> TriadKind for PhantomData<K>
where
    K: TriadKind,
{
    seal!();

    fn class() -> Triads {
        K::class()
    }

    fn name() -> &'static str {
        K::name()
    }
}

pub trait TriadKind: Relative + TriadCls {
    private!();

    fn class() -> Triads
    where
        Self: Sized;

    fn name() -> &'static str
    where
        Self: Sized;
    /// Returns a new instance of [PhantomData](core::marker::PhantomData);
    /// This method is the only possible constructor for these objects,
    /// a charecteristic enfored with 0-variant enum declarations.
    fn phantom() -> core::marker::PhantomData<Self>
    where
        Self: Sized,
    {
        core::marker::PhantomData::<Self>
    }

    fn is_valid(notes: &[Note; 3]) -> bool
    where
        Self: Sized,
    {
        Self::class().validate(notes)
    }

    fn thirds() -> (Third, Third)
    where
        Self: Sized,
    {
        Self::class().thirds()
    }

    fn root_to_third() -> Third
    where
        Self: Sized,
    {
        Self::thirds().0
    }

    fn third_to_fifth() -> Third
    where
        Self: Sized,
    {
        Self::thirds().1
    }

    fn root_to_fifth() -> Fifth
    where
        Self: Sized,
    {
        Self::class().root_to_fifth()
    }

    fn intervals() -> (Third, Third, Fifth)
    where
        Self: Sized,
    {
        let (a, b) = Self::thirds();
        (a, b, Self::root_to_fifth())
    }

    fn is_augmented() -> bool
    where
        Self: Sized,
    {
        Self::class().is_augmented()
    }

    fn is_diminished() -> bool
    where
        Self: Sized,
    {
        Self::class().is_diminished()
    }

    fn is_major() -> bool
    where
        Self: Sized,
    {
        Self::class().is_major()
    }

    fn is_minor() -> bool
    where
        Self: Sized,
    {
        Self::class().is_minor()
    }
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
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub enum Triads {
    Augmented,
    Diminished,
    #[default]
    Major,
    Minor,
}

impl Triads {
    pub fn try_from_notes(
        root: Note,
        third: Note,
        fifth: Note,
    ) -> Result<super::Triad<Self>, TriadError> {
        for i in Self::iter() {
            if i.is_valid(root, third, fifth) {
                let triad = super::Triad {
                    notes: [root, third, fifth],
                    _class: PhantomData::<Self>,
                };
                return Ok(triad);
            }
        }
        Err(TriadError::invalid_triad(
            "The given notes do not form a valid triad...",
        ))
    }
    /// A functional constructor for the [Augmented](Triads::Augmented) class.
    pub fn augmented() -> Self {
        Triads::Augmented
    }
    /// A functional constructor for the [Diminished](Triads::Diminished) class.
    pub fn diminished() -> Self {
        Triads::Diminished
    }
    /// A functional constructor for the [Major](Triads::Major) class.
    pub fn major() -> Self {
        Triads::Major
    }
    /// A functional constructor for the [Minor](Triads::Minor) class.
    pub fn minor() -> Self {
        Triads::Minor
    }

    pub fn is<K: TriadKind>(&self) -> bool {
        K::class() == *self
    }

    pub fn of<K: TriadKind>() -> Self {
        if K::is_major() {
            Triads::major()
        } else if K::is_minor() {
            Triads::minor()
        } else if K::is_augmented() {
            Triads::augmented()
        } else {
            Triads::diminished()
        }
    }

    pub fn classify<K: TriadKind>() -> Self {
        K::class()
    }

    pub fn intervals(&self) -> (Third, Third, Fifth) {
        use Fifth::*;
        use Third::*;
        match self {
            Triads::Augmented => (Major, Major, Augmented),
            Triads::Diminished => (Minor, Minor, Diminished),
            Triads::Major => (Major, Minor, Perfect),
            Triads::Minor => (Minor, Major, Perfect),
        }
    }

    pub fn is_valid(&self, root: Note, third: Note, fifth: Note) -> bool {
        let (a, b) = self.thirds();
        let c = self.root_to_fifth();
        // compute the interval between the root and third
        let rt = {
            let interval = third - root;
            Third::try_from(*interval.pitch())
        };
        // compute the interval between the third and fifth
        let tf = {
            let interval = fifth - third;
            Third::try_from(*interval.pitch())
        };
        // compute the interval between the root and fifth
        let rf = {
            let interval = fifth - root;
            Fifth::try_from(*interval.pitch())
        };

        rt == Ok(a) && tf == Ok(b) && rf == Ok(c)
    }
    /// Returns the [class](Triads) of the triad
    pub fn validate(&self, notes: &[Note; 3]) -> bool {
        // the interval between the root and the third must be a third
        let rt = notes[1] - notes[0];
        // the interval between the third and the fifth must be a third
        let tf = notes[2] - notes[1];

        Third::try_from(*rt.pitch()).is_ok() && Third::try_from(*tf.pitch()).is_ok()
    }

    pub fn thirds(&self) -> (Third, Third) {
        use Third::*;
        match self {
            Triads::Augmented => (Major, Major),
            Triads::Diminished => (Minor, Minor),
            Triads::Major => (Major, Minor),
            Triads::Minor => (Minor, Major),
        }
    }

    pub fn root_to_third(&self) -> Third {
        self.thirds().0
    }

    pub fn third_to_fifth(&self) -> Third {
        self.thirds().1
    }

    pub fn root_to_fifth(&self) -> Fifth {
        use Fifth::*;
        match self {
            Triads::Augmented => Augmented,
            Triads::Diminished => Diminished,
            Triads::Major | Triads::Minor => Perfect,
        }
    }
}

impl<K> From<PhantomData<K>> for Triads
where
    K: TriadKind,
{
    fn from(_: PhantomData<K>) -> Self {
        K::class()
    }
}

impl<K> From<Triads> for PhantomData<K>
where
    K: TriadKind,
{
    fn from(_: Triads) -> Self {
        PhantomData::<K>
    }
}

macro_rules! class {
    (@impl $name:ident::$call:ident -> $relative:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize),)]
        pub struct $name;

        impl $name {
            pub fn class() -> Triads {
                Triads::$name
            }

            pub fn name(&self) -> &'static str {
                stringify!($call)
            }
        }

        impl Relative for $name {
            type Rel = Triads;

            fn relative(&self) -> Self::Rel {
                Triads::$relative
            }
        }

        impl TriadCls for $name {
            seal!();

            fn named(&self) -> &'static str {
                stringify!($call)
            }
        }

        impl TriadKind for $name {
            seal!();

            fn class() -> Triads where Self: Sized {
                Triads::$name
            }

            fn name() -> &'static str where Self: Sized {
                stringify!($call)
            }

            paste::paste! {
                fn [<is_ $call>]() -> bool {
                    true
                }
            }
        }

        unsafe impl Send for $name {}

        unsafe impl Sync for $name {}
    };
    ($($name:ident::$call:ident -> $rel:ident),* $(,)?) => {
        $(
            class!(@impl $name::$call -> $rel);
        )*
    };
}

class!(
    Augmented::augmented -> Diminished,
    Diminished::diminished -> Augmented,
    Major::major -> Minor,
    Minor::minor -> Major
);
