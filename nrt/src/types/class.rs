/*
    Appellation: classes <module>
    Contrib: @FL03
*/
use num_traits::ToPrimitive;
use rstmt::PitchMod;

/// The [`TriadClass`] implementation enumerates the allowed triad classifications determined
/// by the intervals between the chord factors.
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
#[strum(serialize_all = "lowercase")]
pub enum TriadClass {
    #[default]
    Major,
    Minor,
    Augmented,
    Diminished,
}

impl TriadClass {
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
    /// returns the class variant that is _relative_ to the current one
    pub const fn relative(&self) -> Self {
        match self {
            TriadClass::Major => TriadClass::Minor,
            TriadClass::Minor => TriadClass::Major,
            TriadClass::Augmented => TriadClass::Diminished,
            TriadClass::Diminished => TriadClass::Augmented,
        }
    }
    /// returns the intervals corresponding to the triad type as arrays of three `usize` values
    /// ordered as: [root_to_third, root_to_fifth, third_to_fifth]
    pub const fn intervals(&self) -> [usize; 3] {
        match self {
            TriadClass::Major => [4, 7, 3],
            TriadClass::Minor => [3, 7, 4],
            TriadClass::Augmented => [4, 8, 4],
            TriadClass::Diminished => [3, 6, 3],
        }
    }
    /// returns the two third intervals defining the current variant
    pub const fn thirds(&self) -> (usize, usize) {
        (self.root(), self.third())
    }
    /// returns the **interval** between the root and third chord factors
    pub const fn root(&self) -> usize {
        match self {
            TriadClass::Major => 4,
            TriadClass::Minor => 3,
            TriadClass::Augmented => 4,
            TriadClass::Diminished => 3,
        }
    }
    /// returns the **interval** between the third and fifth chord factors
    pub const fn third(&self) -> usize {
        match self {
            TriadClass::Major => 3,
            TriadClass::Minor => 4,
            TriadClass::Augmented => 4,
            TriadClass::Diminished => 3,
        }
    }
    /// returns the **interval** between the root and fifth chord factors
    pub const fn fifth(&self) -> usize {
        match self {
            TriadClass::Major => 7,
            TriadClass::Minor => 7,
            TriadClass::Augmented => 8,
            TriadClass::Diminished => 6,
        }
    }
    /// returns true if the given chord factors satisfy the requirements of the current class
    pub fn is_valid(&self, root: usize, third: usize, fifth: usize) -> bool {
        let [rt, rf, tf] = self.intervals();
        (third - root).pmod() == rt && (fifth - root).pmod() == rf && (fifth - third).pmod() == tf
    }
    /// validate a chord's composition satisfies the requirements of the current class
    pub fn validate<T>(&self, notes: &[T; 3]) -> bool
    where
        T: ToPrimitive,
    {
        let [a, b, c] = self.intervals();
        let r = <T>::to_isize(&notes[0]).unwrap();
        let t = <T>::to_isize(&notes[1]).unwrap();
        let f = <T>::to_isize(&notes[2]).unwrap();
        (t - r).pmod() as usize == a && (f - t).pmod() as usize == c && (f - r).pmod() as usize == b
    }
}

impl crate::TriadCls for TriadClass {
    type Rel = Self;

    seal! {}

    fn new() -> Self {
        Self::default()
    }

    fn is_major(&self) -> bool {
        matches!(self, TriadClass::Major)
    }

    fn is_minor(&self) -> bool {
        matches!(self, TriadClass::Minor)
    }

    fn is_augmented(&self) -> bool {
        matches!(self, TriadClass::Augmented)
    }

    fn is_diminished(&self) -> bool {
        matches!(self, TriadClass::Diminished)
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

impl From<usize> for TriadClass {
    fn from(value: usize) -> Self {
        match value {
            0 => TriadClass::Major,
            1 => TriadClass::Minor,
            2 => TriadClass::Augmented,
            3 => TriadClass::Diminished,
            _ => TriadClass::Major,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_creation() -> crate::Result<()> {
        let class = TriadClass::try_from_arr([0, 4, 7])?;
        assert!(class.is_major());
        let class = TriadClass::try_from_arr([0, 3, 7])?;
        assert!(class.is_minor());
        let class = TriadClass::try_from_arr([0, 4, 8])?;
        assert!(class.is_augmented());
        let class = TriadClass::try_from_arr([0, 3, 6])?;
        assert!(class.is_diminished());

        assert!(TriadClass::try_from_arr([0, 7, 4]).is_err());
        assert!(TriadClass::try_from_arr([0, 5, 9]).is_err());

        Ok(())
    }
}
