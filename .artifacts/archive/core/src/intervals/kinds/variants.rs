/*
    Appellation: variants <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

interval! {
    default: Major;
    pub enum Third {
        Minor = 3,
        Major = 4,
    }
}

interval! {
    default: Perfect;
    pub enum Fourth {
        Perfect = 5,
    }
}

interval! {
    default: Perfect;
    pub enum Fifth {
        Diminished = 6,
        Perfect = 7,
        Augmented = 8,
    }
}

interval! {
    default: Diminished;
    pub enum Seventh {
        Diminished = 9,
        Minor = 10,
        Major = 11,
        Augmented = 12,
    }
}

/*
 ************* Implementations *************
*/
impl Third {
    pub fn minor() -> Self {
        Third::Minor
    }

    pub fn major() -> Self {
        Third::Major
    }
}

impl Fourth {
    pub fn perfect() -> Self {
        Fourth::Perfect
    }
}

impl Fifth {
    pub fn augmented() -> Self {
        Fifth::Augmented
    }

    pub fn diminished() -> Self {
        Fifth::Diminished
    }

    pub fn perfect() -> Self {
        Fifth::Perfect
    }

    pub fn from_thirds(lhs: Third, rhs: Third) -> Self {
        let value = lhs as i8 + rhs as i8;
        match value {
            6 => Fifth::Diminished,
            7 => Fifth::Perfect,
            8 => Fifth::Augmented,
            _ => panic!("Invalid fifth value: {}", value),
        }
    }
}

impl Seventh {
    pub fn augmented() -> Self {
        Seventh::Augmented
    }

    pub fn diminished() -> Self {
        Seventh::Diminished
    }

    pub fn major() -> Self {
        Seventh::Major
    }

    pub fn minor() -> Self {
        Seventh::Minor
    }
}
