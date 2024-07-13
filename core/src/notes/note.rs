/*
    Appellation: note <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{Notable, Octave, Pitch};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Note<P = Pitch> {
    pub(crate) octave: Octave,
    pub(crate) pitch: P,
}

impl<P> Note<P>
where
    P: Notable,
{
    pub fn new(octave: Octave, pitch: P) -> Self {
        Self { octave, pitch }
    }
    /// Returns an owned instance of the note's octave
    pub const fn octave(&self) -> &Octave {
        &self.octave
    }

    pub const fn pitch(&self) -> &P {
        &self.pitch
    }

    pub fn set_octave(&mut self, octave: Octave) {
        self.octave = octave;
    }

    pub fn with_octave(self, octave: Octave) -> Self {
        Self { octave, ..self }
    }

    pub fn with_pitch<P2>(self, pitch: P2) -> Note<P2>
    where
        P2: Notable,
    {
        Note {
            octave: self.octave,
            pitch,
        }
    }
}

impl<P> core::fmt::Display for Note<P>
where
    P: Notable,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}.{}", self.pitch, self.octave)
    }
}

macro_rules! impl_ops {
    (@impl $name:ident$(<$($gen:ident),+>)?, $trait:ident, $fn:ident, $op:tt) => {
        impl<P> core::ops::$trait for Note<P>
        where
            P: $crate::pitch::Notable,
        {
            type Output = Note<P>;
            fn $fn(self, rhs: Self) -> Self::Output {
                Note {
                    octave: self.octave $op rhs.octave,
                    pitch: self.pitch $op rhs.pitch,
                }
            }
        }
    };
    (@impl $name:ident$(<$($gen:ident),+>)?, $trait:ident, $fn:ident, $op:tt, $($rest:tt)*) => {
        impl_ops!(@impl $name$(<$($gen),+>)?, $trait, $fn, $op);
        impl_ops!(@impl $name$(<$($gen),+>)?, $($rest)*);
    };
    ($($rest:tt)*) => {
        impl_ops!(@impl Note, $($rest)*);
    };
}
