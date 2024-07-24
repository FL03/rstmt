/*
    Appellation: classes <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::TriadError;
use core::marker::PhantomData;
use rstmt::{Fifth, Note, Third};
use strum::IntoEnumIterator;

/// This trait denotes privately declared instances of different classes of triads.
/// Traditionally, triads have two primary classes: [major](Major) and [minor](Minor), however, there are
/// two additional classes: [augmented](Augmented) and [diminished](Diminished). This trait is used to determine
pub trait TriadCls {
    private!();
}

pub trait TriadKind: TriadCls
where
    Self: Copy + Sized,
{
    /// Returns a new instance of [PhantomData](core::marker::PhantomData);
    /// This method is the only possible constructor for these objects,
    /// a charecteristic enfored with 0-variant enum declarations.
    fn phantom() -> core::marker::PhantomData<Self> {
        core::marker::PhantomData::<Self>
    }
    fn is_valid(notes: &[Note; 3]) -> bool {
        Self::class().validate(notes)
    }

    fn class() -> Triads {
        if Self::is_major() {
            Triads::major()
        } else if Self::is_minor() {
            Triads::minor()
        } else if Self::is_augmented() {
            Triads::augmented()
        } else {
            Triads::diminished()
        }
    }

    fn thirds() -> (Third, Third) {
        Self::class().thirds()
    }

    fn root_to_third() -> Third {
        Self::thirds().0
    }

    fn third_to_fifth() -> Third {
        Self::thirds().1
    }

    fn root_to_fifth() -> Fifth {
        Self::class().root_to_fifth()
    }

    fn intervals() -> (Third, Third, Fifth) {
        let (a, b) = Self::thirds();
        (a, b, Self::root_to_fifth())
    }

    fn name() -> &'static str;

    fn is_augmented() -> bool {
        false
    }

    fn is_diminished() -> bool {
        false
    }

    fn is_major() -> bool {
        false
    }

    fn is_minor() -> bool {
        false
    }
}

macro_rules! class {
    (@impl $name:ident::$call:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize),)]
        pub enum $name {}

        impl TriadCls for $name {
            seal!();
        }

        impl TriadKind for $name {

            fn name() -> &'static str {
                stringify!($name)
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
    ($($name:ident::$call:ident),* $(,)?) => {
        $(
            class!(@impl $name::$call);
        )*
    };
}

class!(
    Augmented::augmented,
    Diminished::diminished,
    Major::major,
    Minor::minor
);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Class {
    Augmented(PhantomData<Augmented>),
    Diminished(PhantomData<Diminished>),
    Major(PhantomData<Major>),
    Minor(PhantomData<Minor>),
}

impl Class {
    pub fn augmented() -> Self {
        Class::Augmented(PhantomData::<Augmented>)
    }

    pub fn diminished() -> Self {
        Class::Diminished(PhantomData::<Diminished>)
    }

    pub fn major() -> Self {
        Class::Major(PhantomData::<Major>)
    }

    pub fn minor() -> Self {
        Class::Minor(PhantomData::<Minor>)
    }

    pub fn is<K: TriadKind>(&self) -> bool {
        match self {
            Class::Augmented(_) => K::is_augmented(),
            Class::Diminished(_) => K::is_diminished(),
            Class::Major(_) => K::is_major(),
            Class::Minor(_) => K::is_minor(),
        }
    }

    // pub fn into_kind<K>(self) -> PhantomData<K> {
    //     match self {
    //         Class::Augmented(cls) => cls,
    //         Class::Diminished(cls) => cls,
    //         Class::Major(cls) => cls,
    //         Class::Minor(cls) => cls,
    //     }
    // }
}

impl Default for Class {
    fn default() -> Self {
        Class::Major(PhantomData::<Major>)
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

impl TriadCls for Triads {
    seal!();
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
