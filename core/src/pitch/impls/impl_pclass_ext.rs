/*
    Appellation: impl_pclass_ext <module>
    Created At: 2025.12.23:16:16:05
    Contrib: @FL03
*/
use crate::error::Error;
use crate::pitch::pitch_class::PitchClass;
use crate::pitch::traits::{Accidental, PitchClassRepr, RawPitchClass};

impl<P, K> core::fmt::Debug for PitchClass<P, K>
where
    P: RawPitchClass<Tag = K>,
    K: Accidental,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.is_natural() {
            write!(f, "{}", self.get())
        } else {
            write!(f, "{}{}", self.get(), self.kind.symbol())
        }
    }
}

impl<P, K> core::fmt::Display for PitchClass<P, K>
where
    P: RawPitchClass<Tag = K>,
    K: Accidental,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.is_natural() {
            write!(f, "{}", self.get())
        } else {
            write!(f, "{}{}", self.get(), self.kind.symbol())
        }
    }
}

impl<P, K> core::str::FromStr for PitchClass<P, K>
where
    P: PitchClassRepr<Tag = K>,
    K: Accidental + core::str::FromStr<Err = Error>,
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lex_class, lex_kind) = if s.len() > 1 {
            let (head, tail) = s.split_at(s.len() - 1);
            (head, tail)
        } else {
            (s, "Natural")
        };
        let class = lex_class.parse::<P>()?;
        let kind = lex_kind.parse::<K>()?;
        Ok(Self { class, kind })
    }
}
impl<P, K> TryFrom<isize> for PitchClass<P, K>
where
    P: PitchClassRepr<Tag = K>,
    K: Accidental,
{
    type Error = Error;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if P::is(value) {
            return Ok(Self {
                class: P::new(),
                kind: K::new(),
            });
        }
        Err(Error::MismatchedPitchClasses(value, <P>::IDX))
    }
}

impl<P, K> AsRef<isize> for PitchClass<P, K>
where
    P: RawPitchClass<Tag = K>,
    K: Accidental,
{
    fn as_ref(&self) -> &isize {
        self.get().as_ref()
    }
}

impl<P, K> AsRef<str> for PitchClass<P, K>
where
    P: RawPitchClass<Tag = K>,
    K: Accidental,
{
    fn as_ref(&self) -> &str {
        self.get().name()
    }
}

impl<P, K> core::borrow::Borrow<isize> for PitchClass<P, K>
where
    P: RawPitchClass<Tag = K>,
    K: Accidental,
{
    fn borrow(&self) -> &isize {
        self.get().borrow()
    }
}

impl<P, K> core::ops::Deref for PitchClass<P, K>
where
    P: RawPitchClass<Tag = K>,
    K: Accidental,
{
    type Target = P;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

unsafe impl<P, K> Send for PitchClass<P, K>
where
    P: RawPitchClass<Tag = K>,
    K: Accidental,
{
}

unsafe impl<P, K> Sync for PitchClass<P, K>
where
    P: RawPitchClass<Tag = K>,
    K: Accidental,
{
}

impl<P, K> PartialEq<isize> for PitchClass<P, K>
where
    P: RawPitchClass<Tag = K>,
    K: Accidental,
{
    fn eq(&self, other: &isize) -> bool {
        self.get().index() == *other
    }
}

impl<P, K> PartialEq<PitchClass<P, K>> for isize
where
    P: RawPitchClass<Tag = K>,
    K: Accidental,
{
    fn eq(&self, other: &PitchClass<P, K>) -> bool {
        *self == other.get().index()
    }
}
