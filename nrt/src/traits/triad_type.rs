/*
    Appellation: triad_kind <module>
    Created At: 2025.12.23:14:15:46
    Contrib: @FL03
*/
use crate::Triads;
/// The [`Relative`] trait is used to define a relationship between two distinct triad
/// classifications
pub trait Relative {
    type Rel;

    private! {}

    fn rel(&self) -> Self::Rel;
}

/// The [`TriadType`] trait is used to represent the various classifications of a triad
/// considered by the Neo-Riemannian theory.
pub trait TriadType
where
    Self: 'static + Copy + Relative + Send + Sync + core::fmt::Debug + core::fmt::Display,
{
    private! {}

    fn new() -> Self
    where
        Self: Sized;

    fn root(&self) -> usize;

    fn fifth(&self) -> usize;

    fn third(&self) -> usize;
    /// consumes the instance, returning a variant of the [`TriadClass`] enum
    fn dynamic(&self) -> crate::Triads {
        if self.is_major() {
            Triads::Major
        } else if self.is_minor() {
            Triads::Minor
        } else if self.is_augmented() {
            Triads::Augmented
        } else if self.is_diminished() {
            Triads::Diminished
        } else {
            unreachable!("invalid triad type")
        }
    }

    fn is_major(&self) -> bool {
        false
    }

    fn is_minor(&self) -> bool {
        false
    }

    fn is_augmented(&self) -> bool {
        false
    }

    fn is_diminished(&self) -> bool {
        false
    }
}

pub trait RelTriad: TriadType + Relative
where
    Self::Rel: TriadType,
{
    private! {}

    fn relative(&self) -> Self::Rel;
}

/*
 ************* Implementations *************
*/

impl<A, B> RelTriad for A
where
    A: TriadType + Relative<Rel = B>,
    B: TriadType,
{
    seal! {}

    fn relative(&self) -> B {
        <B>::new()
    }
}

impl Relative for Triads {
    type Rel = Triads;

    seal! {}

    fn rel(&self) -> Self::Rel {
        self.relative()
    }
}

impl TriadType for Triads {
    seal! {}

    fn new() -> Self {
        Self::default()
    }

    fn is_major(&self) -> bool {
        matches!(self, Triads::Major)
    }

    fn is_minor(&self) -> bool {
        matches!(self, Triads::Minor)
    }

    fn is_augmented(&self) -> bool {
        matches!(self, Triads::Augmented)
    }

    fn is_diminished(&self) -> bool {
        matches!(self, Triads::Diminished)
    }

    fn root(&self) -> usize {
        self.root()
    }

    fn fifth(&self) -> usize {
        self.fifth()
    }

    fn third(&self) -> usize {
        self.third()
    }
}

macro_rules! triad_kind {
    (impl $trait:ident for {$($($name:ident)::*<Rel = $rel:ty>::<[$($v:literal),* $(,)?]> $({$($rest:tt)*})?),* $(,)?}) => {
        $(triad_kind! { @impl $trait for $($name)::*<Rel = $rel>::<[$($v),*]> $({$($rest)*})? })*
    };
    (@impl $trait:ident for $($name:ident)::* <Rel = $rel:ty>::<[$r:literal, $f:literal, $t:literal]> $({$($rest:tt)*})?) => {
        impl $trait for $($name)::* {
            // type Rel = $rel;

            seal! {}

            fn new() -> Self {
                Self::default()
            }

            fn root(&self) -> usize {
                $r
            }

            fn fifth(&self) -> usize {
                $f
            }

            fn third(&self) -> usize {
                $t
            }

            $($($rest)*)?
        }

        impl Relative for $($name)::* {
            type Rel = $rel;

            seal! {}

            fn rel(&self) -> Self::Rel {
                <$rel>::default()
            }
        }
    };
}

triad_kind! {
    impl TriadType for {
        rstmt_core::Augmented<Rel = rstmt_core::Diminished>::<[4, 8, 4]> {
            fn is_augmented(&self) -> bool {
                true
            }
        },
        rstmt_core::Diminished<Rel = rstmt_core::Augmented>::<[3, 6, 3]> {
            fn is_diminished(&self) -> bool {
                true
            }
        },
        rstmt_core::Major<Rel = rstmt_core::Minor>::<[4, 7, 3]> {
            fn is_major(&self) -> bool {
                true
            }
        },
        rstmt_core::Minor<Rel = rstmt_core::Major>::<[3, 7, 4]> {
            fn is_minor(&self) -> bool {
                true
            }
        }
    }
}
