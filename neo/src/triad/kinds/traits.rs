/*
    Appellation: traits <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::triad::Triads;
use core::marker::PhantomData;
use rstmt::{Fifth, Note, Third};

/// This trait denotes privately declared instances of different classes of triads.
/// Traditionally, triads have two primary classes: [major](Major) and [minor](Minor), however, there are
/// two additional classes: [augmented](Augmented) and [diminished](Diminished). This trait is used to determine
pub trait TriadCls {
    private!();

    fn named(&self) -> &'static str;
}
///
///
///
pub trait Relative {
    type Rel;

    fn relative(&self) -> Self::Rel;
}

pub trait Kind {
    type Class;

    seal!();

    fn class() -> Self::Class where Self: Sized;

    fn name() -> &'static str where Self: Sized;
}
///
///
///
pub trait TriadKind: Kind<Class = Triads> {
    type Rel: TriadKind;
    
    private!();

    // fn class() -> Triads
    // where
    //     Self: Sized;

    // fn name() -> &'static str
    // where
    //     Self: Sized;

    /// Returns a new instance of [PhantomData]; This method is the only possible constructor
    /// for these objects, a charecteristic enfored with 0-variant enum declarations.
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

/*
 ************* Implementations *************
*/

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

impl<K: Kind> Kind for PhantomData<K> {
    type Class = K::Class;

    seal!();

    fn class() -> Self::Class {
        K::class()
    }

    fn name() -> &'static str {
        K::name()
    }
}

impl<K: TriadKind> TriadKind for PhantomData<K> {
    type Rel = K::Rel;

    seal!();
}
