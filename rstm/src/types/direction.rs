/*
    Appellation: direction <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait IntoDirection {
    fn into_direction(self) -> Direction;
}

impl IntoDirection for i8 {
    fn into_direction(self) -> Direction {
        Direction::from(self)
    }
}

impl IntoDirection for char {
    fn into_direction(self) -> Direction {
        Direction::from(self)
    }
}

impl IntoDirection for &str {
    fn into_direction(self) -> Direction {
        Direction::from(self)
    }
}

/// [Direction] enumerates the various directions a head can move, namely: left, right, and stay.
/// The included methods and implementations aim to streamline the conversion between [Direction] and other types.
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
    strum::EnumIter,
    strum::VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    rename_all = "lowercase"
)]
#[repr(i8)]
#[strum(serialize_all = "lowercase")]
pub enum Direction {
    #[cfg_attr(
        feature = "serde",
        serde(
            alias = "left",
            alias = "l",
            alias = "L",
            alias = "LEFT",
            alias = "Left"
        )
    )]
    Left = -1,
    Right = 1,
    #[default]
    Stay = 0,
}

impl Direction {
    /// A functional constructor for [Direction::Left].
    pub fn left() -> Self {
        Self::Left
    }
    /// A functional constructor for [Direction::Right].
    pub fn right() -> Self {
        Self::Right
    }
    /// A functional constructor for [Direction::Stay].
    pub fn stay() -> Self {
        Self::Stay
    }

    pub fn apply(&self, position: usize) -> usize {
        match self {
            Self::Left => position - 1,
            Self::Right => position + 1,
            Self::Stay => position,
        }
    }
    /// Converts an [i8] value into a [Direction] by taking the modulus of the value.
    /// The function uses a modulator of 2 to determine the direction since there are
    /// only two actionable directions ([left](Direction::Left) and [right](Direction::Right)).
    pub fn from_i8(value: i8) -> Self {
        match value % 2 {
            -1 => Self::Left,
            1 => Self::Right,
            _ => Self::Stay,
        }
    }
    /// Converts a [char] value into a direction; matches the value to the corresponding direction.
    pub fn from_char(value: char) -> Self {
        match value {
            'L' | 'l' => Self::Left,
            'R' | 'r' => Self::Right,
            _ => Self::Stay,
        }
    }

    pub fn into_char(self) -> char {
        match self {
            Self::Left => 'L',
            Self::Right => 'R',
            Self::Stay => 'S',
        }
    }
}

impl From<i8> for Direction {
    fn from(value: i8) -> Self {
        match value % 2 {
            -1 => Self::Left,
            0 => Self::Stay,
            1 => Self::Right,
            _ => Self::Stay,
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' | 'l' => Self::Left,
            'R' | 'r' => Self::Right,
            _ => Self::Stay,
        }
    }
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "left" | "Left" | "LEFT" | "l" | "L" => Self::Left,
            "right" | "Right" | "RIGHT" | "r" | "R" => Self::Right,
            _ => Self::Stay,
        }
    }
}
