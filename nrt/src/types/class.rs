/*
    Appellation: classes <module>
    Contrib: @FL03
*/
use rstmt::PitchMod;

// Expanded triad types in Neo-Riemannian theory
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
    pub fn try_from_notes(a: usize, b: usize, c: usize) -> crate::Result<Self> {
        let (a, b, c) = (a as isize, b as isize, c as isize);
        let rt = (b - a).pmod();
        let tf = (c - b).pmod();
        let rf = (c - a).pmod();
        if matches!(rt, 3 | 4) && matches!(tf, 3 | 4) && matches!(rf, 6..=8) && rt + tf == rf {
            let class = match [rt, tf, rf] {
                [4, 3, 7] => TriadClass::Major,
                [3, 4, 7] => TriadClass::Minor,
                [4, 4, 8] => TriadClass::Augmented,
                [3, 3, 6] => TriadClass::Diminished,
                _ => unreachable!(),
            };
            return Ok(class);
        }
        Err(crate::TriadError::InvalidTriad)
    }
    /// try to determine the class of a triad from an array of three notes
    pub fn try_from_arr(arr: [usize; 3]) -> crate::Result<Self> {
        use itertools::Itertools;
        let mut res = Err(crate::TriadError::InvalidTriadClass);
        for (&a, &b, &c) in arr
            .iter()
            .circular_tuple_windows()
            .chain(arr.iter().rev().circular_tuple_windows())
        {
            if let Ok(class) = Self::try_from_notes(a, b, c) {
                res = Ok(class);
                break;
            }
        }
        res
    }
    /// get the relative triad type
    pub fn relative(self) -> Self {
        match self {
            TriadClass::Major => TriadClass::Minor,
            TriadClass::Minor => TriadClass::Major,
            TriadClass::Augmented => TriadClass::Diminished,
            TriadClass::Diminished => TriadClass::Augmented,
        }
    }
    /// returns the intervals corresponding to the triad type
    pub fn intervals(self) -> [usize; 3] {
        match self {
            TriadClass::Major => [4, 3, 7],
            TriadClass::Minor => [3, 4, 7],
            TriadClass::Augmented => [4, 4, 8],
            TriadClass::Diminished => [3, 3, 6],
        }
    }
    pub fn thirds(&self) -> (usize, usize) {
        use TriadClass::*;
        match self {
            Augmented => (4, 4),
            Diminished => (3, 3),
            Major => (4, 3),
            Minor => (3, 4),
        }
    }
    /// returns the interval from the root to the third chord factor; defined by the class
    pub fn root_to_third(self) -> usize {
        self.intervals()[0]
    }
    /// returns the interval from the third to the fifth chord factor; defined by the class
    pub fn third_to_fifth(self) -> usize {
        self.intervals()[1]
    }
    /// returns the interval from the root to the fifth chord factor; defined by the class
    pub fn root_to_fifth(self) -> usize {
        self.intervals()[2]
    }
    pub fn is_valid(&self, root: usize, third: usize, fifth: usize) -> bool {
        let [a, b, c] = self.intervals();
        // compute the interval between the root and third
        let rt = third - root;
        // compute the interval between the third and fifth
        let tf = fifth - third;
        // compute the interval between the root and fifth
        let rf = fifth - root;

        rt == a && tf == b && rf == c
    }
    /// validate a chord's composition satisfies the requirements of the current class
    pub fn validate(&self, notes: &[usize; 3]) -> bool {
        let [a, b, c] = self.intervals();
        let r = notes[0] as isize;
        let t = notes[1] as isize;
        let f = notes[2] as isize;
        (t - r).pmod() as usize == a && (f - t).pmod() as usize == b && (f - r).pmod() as usize == c
    }
}

impl crate::TriadCls for TriadClass {
    seal! {}

    fn new() -> Self {
        Self::default()
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
    fn test_class_creation() {
        let class = TriadClass::try_from_arr([0, 4, 7]).unwrap();
        assert!(class.is_major());
        let class = TriadClass::try_from_arr([0, 3, 7]).unwrap();
        assert!(class.is_minor());
        let class = TriadClass::try_from_arr([0, 4, 8]).unwrap();
        assert!(class.is_augmented());
        let class = TriadClass::try_from_arr([0, 3, 6]).unwrap();
        assert!(class.is_diminished());

        let class = TriadClass::try_from_arr([0, 7, 4]).unwrap();
        assert!(class.is_major());
        let class = TriadClass::try_from_arr([0, 7, 3]).unwrap();
        assert!(class.is_minor());
        let class = TriadClass::try_from_arr([8, 0, 4]).unwrap();
        assert!(class.is_augmented());
        let class = TriadClass::try_from_arr([6, 0, 3]).unwrap();
        assert!(class.is_diminished());
    }
}
