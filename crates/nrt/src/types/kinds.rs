/*
    Appellation: classes <module>
    Contrib: @FL03
*/
use crate::traits::TriadType;
use num_traits::{FromPrimitive, ToPrimitive};
use rstmt::PitchMod;

/// The [`Triads`] implementation enumerates the allowed triad classifications determined
/// by the intervals between the chord factors.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialOrd,
    strum::AsRefStr,
    strum::Display,
    strum::EnumIs,
    strum::EnumIter,
    strum::EnumString,
    strum::VariantArray,
    strum::VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(untagged, rename_all = "lowercase")
)]
#[strum(serialize_all = "lowercase")]
pub enum Triads {
    #[default]
    Major = 0,
    Minor = 1,
    Augmented = 2,
    Diminished = 3,
}

impl Triads {
    pub fn from_class<C>(class: C) -> Self
    where
        C: TriadType,
    {
        if class.is_major() {
            Self::Major
        } else if class.is_minor() {
            Self::Minor
        } else if class.is_augmented() {
            Self::Augmented
        } else if class.is_diminished() {
            Self::Diminished
        } else {
            panic!("invalid triad class")
        }
    }
    pub fn is<T: TriadType>(&self, class: T) -> bool {
        match self {
            Triads::Major => class.is_major(),
            Triads::Minor => class.is_minor(),
            Triads::Augmented => class.is_augmented(),
            Triads::Diminished => class.is_diminished(),
        }
    }
    /// a functional constructor for the [`Major`](Triads::Major) variant
    pub const fn major() -> Self {
        Self::Major
    }
    /// a functional constructor for the [`Minor`](Triads::Minor) variant
    pub const fn minor() -> Self {
        Self::Minor
    }
    /// a functional constructor for the [`Augmented`](Triads::Augmented) variant
    pub const fn augmented() -> Self {
        Self::Augmented
    }
    /// a functional constructor for the [`Diminished`](Triads::Diminished) variant
    pub const fn diminished() -> Self {
        Self::Diminished
    }
    /// try to derive a classification for a triad from three notes
    pub fn try_from_notes(a: isize, b: isize, c: isize) -> crate::Result<Self> {
        Self::try_from_arr([a, b, c])
    }
    #[cfg(feature = "alloc")]
    /// try to determine the class of a triad from an array of three notes
    pub fn try_from_arr(notes: [isize; 3]) -> crate::Result<Self> {
        use itertools::Itertools;
        let intervals = notes
            .iter()
            .combinations(2)
            .map(|v| (v[1] - v[0]).pmod())
            .collect::<Vec<_>>();
        match intervals[..] {
            [4, 7, 3] => Ok(Self::Major),
            [3, 7, 4] => Ok(Self::Minor),
            [4, 8, 4] => Ok(Self::Augmented),
            [3, 6, 3] => Ok(Self::Diminished),
            _ => Err(crate::TriadError::InvalidTriad),
        }
    }
    // TODO: ensure augmented & diminished are handled correctly
    /// returns the class _relative_ to the current variant; for major and minor triads its
    /// rather straightforward
    pub const fn relative(&self) -> Self {
        match self {
            Triads::Major => Triads::Minor,
            Triads::Minor => Triads::Major,
            Triads::Augmented => Triads::Diminished,
            Triads::Diminished => Triads::Augmented,
        }
    }
    /// returns the intervals corresponding to the triad type as arrays of three `usize` values
    /// ordered as: [root_to_third, root_to_fifth, third_to_fifth]
    pub const fn intervals(&self) -> [usize; 3] {
        match self {
            Triads::Major => [4, 7, 3],
            Triads::Minor => [3, 7, 4],
            Triads::Augmented => [4, 8, 4],
            Triads::Diminished => [3, 6, 3],
        }
    }
    /// returns the two third intervals defining the current variant
    pub const fn thirds(&self) -> (usize, usize) {
        (self.root(), self.third())
    }
    /// returns the **interval** between the root and third chord factors
    pub const fn root(&self) -> usize {
        match self {
            Triads::Major => 4,
            Triads::Minor => 3,
            Triads::Augmented => 4,
            Triads::Diminished => 3,
        }
    }
    /// returns a reference to the **interval** between the root and third chord factors
    pub const fn root_ref(&self) -> &usize {
        match self {
            Triads::Major => &4,
            Triads::Minor => &3,
            Triads::Augmented => &4,
            Triads::Diminished => &3,
        }
    }
    /// returns the **interval** between the third and fifth chord factors
    pub const fn third(&self) -> usize {
        match self {
            Triads::Major => 3,
            Triads::Minor => 4,
            Triads::Augmented => 4,
            Triads::Diminished => 3,
        }
    }
    /// returns a reference to the **interval** between the third and fifth chord factors
    pub const fn third_ref(&self) -> &usize {
        match self {
            Triads::Major => &3,
            Triads::Minor => &4,
            Triads::Augmented => &4,
            Triads::Diminished => &3,
        }
    }
    /// returns the **interval** between the root and fifth chord factors
    pub const fn fifth(&self) -> usize {
        match self {
            Triads::Major => 7,
            Triads::Minor => 7,
            Triads::Augmented => 8,
            Triads::Diminished => 6,
        }
    }
    /// returns a reference to the **interval** between the root and fifth chord factors
    pub const fn fifth_ref(&self) -> &usize {
        match self {
            Triads::Major => &7,
            Triads::Minor => &7,
            Triads::Augmented => &8,
            Triads::Diminished => &6,
        }
    }
    /// returns true if the given chord factors satisfy the requirements of the current class
    pub fn is_valid(&self, root: usize, third: usize, fifth: usize) -> bool {
        let [rt, rf, tf] = self.intervals();
        (third - root).pmod() == rt && (fifth - root).pmod() == rf && (fifth - third).pmod() == tf
    }
    /// validate a chord's composition satisfies the requirements of the current class
    pub fn validate<T>(&self, &[r, t, f]: &[T; 3]) -> bool
    where
        T: Copy
            + PartialEq
            + FromPrimitive
            + ToPrimitive
            + PitchMod<Output = T>
            + core::ops::Sub<Output = T>,
    {
        let a = T::from_usize(self.root()).unwrap();
        let b = T::from_usize(self.fifth()).unwrap();
        let c = T::from_usize(self.third()).unwrap();
        (t - r).pmod() == a && (f - t).pmod() == c && (f - r).pmod() == b
    }
}

macro_rules! impl_from_triad_class {
    ($($T:ty),* $(,)?) => {
        $(
            impl From<$T> for Triads {
                fn from(value: $T) -> Self {
                    match value % 4 {
                        0 => Triads::Major,
                        1 => Triads::Minor,
                        2 => Triads::Augmented,
                        3 => Triads::Diminished,
                        _ => unreachable! { "invalid modulo operation" },
                    }
                }
            }

            impl From<Triads> for $T {
                fn from(value: Triads) -> Self {
                    value as $T
                }
            }
        )*
    };
}

impl_from_triad_class! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

impl core::ops::Index<super::Factors> for Triads {
    type Output = usize;

    fn index(&self, index: super::Factors) -> &Self::Output {
        match index {
            super::Factors::Root => self.root_ref(),
            super::Factors::Third => self.third_ref(),
            super::Factors::Fifth => self.fifth_ref(),
        }
    }
}

macro_rules! interval_to_class {
    (@impl $T:ident) => {
            // impl PartialEq<rstmt_core::$T> for Triads {
            //     fn eq(&self, _other: &rstmt_core::$T) -> bool {
            //         matches! { self, Triads::$T }
            //     }
            // }

            impl From<rstmt_core::$T> for Triads {
                fn from(_value: rstmt_core::$T) -> Self {
                    Triads::$T
                }
            }

            impl TryFrom<Triads> for rstmt_core::$T {
                type Error = $crate::TriadError;

                fn try_from(value: Triads) -> Result<Self, Self::Error> {
                    if matches!(value, Triads::$T) {
                        Ok(Self)
                    } else {
                        Err($crate::TriadError::IncompatibleTriadClasses)
                    }
                }
            }
    };
    ($($T:ident),* $(,)?) => {
        $(interval_to_class! { @impl $T })*
    };
}

interval_to_class! { Major, Minor, Augmented, Diminished }

impl<C: crate::TriadType> PartialEq<C> for Triads {
    fn eq(&self, other: &C) -> bool {
        match self {
            Triads::Major => other.is_major(),
            Triads::Minor => other.is_minor(),
            Triads::Augmented => other.is_augmented(),
            Triads::Diminished => other.is_diminished(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_creation() -> crate::Result<()> {
        let class = Triads::try_from_arr([0, 4, 7])?;
        assert!(class.is_major());
        let class = Triads::try_from_arr([0, 3, 7])?;
        assert!(class.is_minor());
        let class = Triads::try_from_arr([0, 4, 8])?;
        assert!(class.is_augmented());
        let class = Triads::try_from_arr([0, 3, 6])?;
        assert!(class.is_diminished());

        assert!(Triads::try_from_arr([0, 7, 4]).is_err());
        assert!(Triads::try_from_arr([0, 5, 9]).is_err());

        Ok(())
    }
}
