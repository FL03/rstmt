/*
    Appellation: pair <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Pair<A, B = A>(A, B);

impl<A, B> Pair<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Self(a, b)
    }

    pub fn from_tuple((a, b): (A, B)) -> Self {
        Self(a, b)
    }

    pub const fn lhs(&self) -> &A {
        &self.0
    }

    pub fn lhs_mut(&mut self) -> &mut A {
        &mut self.0
    }

    pub const fn rhs(&self) -> &B {
        &self.1
    }

    pub fn rhs_mut(&mut self) -> &mut B {
        &mut self.1
    }
}

impl<A> Pair<A, A> {
    pub fn from_iter<I>(mut iter: I) -> Option<Self>
    where
        I: Iterator<Item = A>,
    {
        let a = iter.next()?;
        let b = iter.next()?;
        Some(Self(a, b))
    }
}
