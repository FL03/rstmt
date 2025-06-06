/*
    Appellation: classes <module>
    Contrib: @FL03
*/
use itertools::Itertools;
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
    scsys::VariantConstructors,
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
    derive(serde_derive::Deserialize, serde_derive::Serialize),
    serde(rename_all = "lowercase")
)]
#[strum(serialize_all = "lowercase")]
pub enum Triads {
    #[default]
    Major,
    Minor,
    Augmented,
    Diminished,
}

impl Triads {
    pub fn try_from_notes(a: usize, b: usize, c: usize) -> crate::Result<Self> {
        let (a, b, c) = (a as isize, b as isize, c as isize);
        let rt = (b - a).pmod();
        let tf = (c - b).pmod();
        let rf = (c - a).pmod();
        if matches!(rt, 3 | 4) && matches!(tf, 3 | 4) && matches!(rf, 6..=8) && rt + tf == rf {
            let class = match [rt, tf, rf] {
                [4, 3, 7] => Triads::Major,
                [3, 4, 7] => Triads::Minor,
                [4, 4, 8] => Triads::Augmented,
                [3, 3, 6] => Triads::Diminished,
                _ => unreachable!(),
            };
            return Ok(class);
        }
        Err(crate::TriadError::InvalidIntervals(format!(
            "{}-{}-{}",
            rt, tf, rf
        )))
    }
    pub fn try_from_arr(arr: [usize; 3]) -> crate::Result<Self> {
        let mut res = Err(crate::TriadError::InvalidIntervals(
            "the given chord does not match any triad class".to_string(),
        ));
        for (&a, &b, &c) in arr
            .iter()
            .circular_tuple_windows()
            .chain(arr.iter().rev().circular_tuple_windows())
        {
            if let Ok(class) = Triads::try_from_notes(a, b, c) {
                res = Ok(class);
                break;
            }
        }
        res
    }
    /// get the relative triad type
    pub fn relative(self) -> Self {
        match self {
            Triads::Major => Triads::Minor,
            Triads::Minor => Triads::Major,
            Triads::Augmented => Triads::Diminished,
            Triads::Diminished => Triads::Augmented,
        }
    }
    /// returns the intervals corresponding to the triad type
    pub fn intervals(self) -> [usize; 3] {
        match self {
            Triads::Major => [4, 3, 7],
            Triads::Minor => [3, 4, 7],
            Triads::Augmented => [4, 4, 8],
            Triads::Diminished => [3, 3, 6],
        }
    }
    pub fn thirds(&self) -> (usize, usize) {
        use Triads::*;
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

impl From<usize> for Triads {
    fn from(value: usize) -> Self {
        match value {
            0 => Triads::Major,
            1 => Triads::Minor,
            2 => Triads::Augmented,
            3 => Triads::Diminished,
            _ => Triads::Major,
        }
    }
}
