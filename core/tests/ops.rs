/*
    Appellation: ops <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate rstmt_core as rstmt;

const MOD: i8 = 12;

lazy_static::lazy_static! {
    static ref CASES: Vec<res::Pair<i8, i8>> = PAIRS.iter().map(|(i, j)| res::Pair::new(*i, *j)).collect();
}
const PAIRS: [(i8, i8); 6] = [(5, 5), (-17, 7), (0, 0), (12, 0), (-12, 0), (-13, 11)];

#[test]
fn test_absmod() {
    use rstmt::ops::AbsMod;
    let cases = PAIRS;
    for (i, j) in cases.iter() {
        assert_eq!(i.absmod(MOD), *j);
    }
}

#[test]
fn test_pymod() {
    use rstmt::ops::PyMod;
    assert_eq!(5.pymod(MOD), 5);
    assert_eq!((-17).pymod(MOD), 7);
    assert_eq!(17.pymod(-MOD), -7);
}

pub mod res {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    pub struct Pair<A, B = A>(pub A, pub B);

    impl<A, B> Pair<A, B> {
        pub fn new(a: A, b: B) -> Self {
            Self(a, b)
        }

        pub fn as_tuple(&self) -> (&A, &B) {
            (&self.0, &self.1)
        }

        pub fn into_tuple(self) -> (A, B) {
            (self.0, self.1)
        }

        pub const fn a(&self) -> &A {
            &self.0
        }

        pub const fn b(&self) -> &B {
            &self.1
        }

        pub fn is_eq(&self) -> bool
        where
            A: PartialEq<B>,
        {
            self.a() == self.b()
        }

        pub fn is_ne(&self) -> bool
        where
            A: PartialEq<B>,
        {
            self.a() != self.b()
        }

        pub fn is_ge(&self) -> bool
        where
            A: PartialOrd<B>,
        {
            self.a() >= self.b()
        }

        pub fn is_gt(&self) -> bool
        where
            A: PartialOrd<B>,
        {
            self.a() > self.b()
        }

        pub fn is_le(&self) -> bool
        where
            A: PartialOrd<B>,
        {
            self.a() <= self.b()
        }

        pub fn is_lt(&self) -> bool
        where
            A: PartialOrd<B>,
        {
            self.a() < self.b()
        }

        pub fn is_default(&self) -> bool
        where
            A: PartialEq + Default,
        {
            self.a() == &A::default()
        }
    }

    impl<A, B> From<(A, B)> for Pair<A, B> {
        fn from(pair: (A, B)) -> Self {
            Self(pair.0, pair.1)
        }
    }
}
