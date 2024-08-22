/*
    Appellation: kinds <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{classes::*, traits::*};

mod classes;
mod traits;

use crate::NeoError;
use core::marker::PhantomData;
use rstmt::{Fifth, Note, Third};

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
    pub fn classify<K: 'static>() -> Self {
        use core::any::TypeId;
        if TypeId::of::<K>() == TypeId::of::<Major>() {
            Triads::Major
        } else if TypeId::of::<K>() == TypeId::of::<Minor>() {
            Triads::Minor
        } else if TypeId::of::<K>() == TypeId::of::<Augmented>() {
            Triads::Augmented
        } else {
            Triads::Diminished
        }
    }
    pub fn try_from_notes(root: Note, third: Note, fifth: Note) -> Result<Self, NeoError> {
        use strum::IntoEnumIterator;
        for i in Self::iter() {
            if i.is_valid(root, third, fifth) {
                return Ok(i);
            }
        }
        Err(NeoError::invalid_triad(
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

    pub fn is<K>(&self) -> bool
    where
        K: Kind + 'static,
    {
        core::any::TypeId::of::<K::Class>() == core::any::TypeId::of::<Self>()
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
    /// Determines if the given notes form a valid triad.
    pub fn is_valid(&self, root: Note, third: Note, fifth: Note) -> bool {
        let (a, b) = self.thirds();
        let c = self.root_to_fifth();
        // compute the interval between the root and third
        let rt = Third::new(root, third);
        // compute the interval between the third and fifth
        let tf = Third::new(third, fifth);
        // compute the interval between the root and fifth
        let rf = Fifth::new(root, fifth);

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
    /// Returns the interval between the root and third as well as the third and fifth.
    pub fn edges(&self) -> (Third, Third, Fifth) {
        use Fifth::*;
        use Third::*;
        match self {
            Triads::Augmented => (Major, Major, Augmented),
            Triads::Diminished => (Minor, Minor, Diminished),
            Triads::Major => (Major, Minor, Perfect),
            Triads::Minor => (Minor, Major, Perfect),
        }
    }
    /// Returns the interval between the root and third as well as the third and fifth.
    pub fn thirds(&self) -> (Third, Third) {
        use Third::*;
        match self {
            Triads::Augmented => (Major, Major),
            Triads::Diminished => (Minor, Minor),
            Triads::Major => (Major, Minor),
            Triads::Minor => (Minor, Major),
        }
    }
    /// Returns the interval between the root and third chord factors.
    pub fn root_to_third(&self) -> Third {
        match self {
            Triads::Augmented | Triads::Major => Third::Major,
            _ => Third::Minor,
        }
    }
    /// Returns the interval between the third and fifth chord factors.
    pub fn third_to_fifth(&self) -> Third {
        match self {
            Triads::Augmented | Triads::Minor => Third::Major,
            _ => Third::Minor,
        }
    }
    /// Returns the interval between the root and fifth chord factors.
    pub fn root_to_fifth(&self) -> Fifth {
        match self {
            Triads::Augmented => Fifth::Augmented,
            Triads::Diminished => Fifth::Diminished,
            _ => Fifth::Perfect,
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
