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
    /// Returns a mutable reference to the note's octave
    pub fn octave_mut(&mut self) -> &mut Octave {
        &mut self.octave
    }
    /// Returns an owned instance of the note's pitch
    pub const fn pitch(&self) -> &P {
        &self.pitch
    }
    /// Returns a mutable reference to the note's pitch
    pub fn pitch_mut(&mut self) -> &mut P {
        &mut self.pitch
    }
    /// Sets the note's octave
    pub fn set_octave(&mut self, octave: Octave) {
        self.octave = octave;
    }
    /// Sets the note's pitch
    pub fn set_pitch(&mut self, pitch: P) {
        self.pitch = pitch;
    }
    /// Returns a new instance of the note with the given octave
    pub fn with_octave(self, octave: Octave) -> Self {
        Self { octave, ..self }
    }
    /// Returns a new instance of the note with the given pitch
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

unsafe impl<P> Send for Note<P> where P: Send {}

unsafe impl<P> Sync for Note<P> where P: Sync {}

macro_rules! impl_std_ops {
    (@impl $trait:ident.$call:ident($rhs:ty) -> $out:ty) => {
        impl<P> core::ops::$trait<$rhs> for Note<P>
        where
            P: $crate::Notable + core::ops::$trait<Output = P>,
        {
            type Output = $out;

            fn $call(self, rhs: $rhs) -> Self::Output {
                Note {
                    octave: ::core::ops::$trait::$call(self.octave, rhs.octave),
                    pitch: ::core::ops::$trait::$call(self.pitch, rhs.pitch)
                }
            }
        }

        impl<'a, P> core::ops::$trait<&'a $rhs> for Note<P>
        where
            P: $crate::Notable + core::ops::$trait<Output = P>,
        {
            type Output = $out;

            fn $call(self, rhs: &'a $rhs) -> Self::Output {
                Note {
                    octave: ::core::ops::$trait::$call(self.octave, rhs.octave),
                    pitch: ::core::ops::$trait::$call(self.pitch, rhs.pitch)
                }
            }
        }

        impl<'a, P> core::ops::$trait<$rhs> for &'a Note<P>
        where
            P: $crate::Notable + core::ops::$trait<Output = P>,
        {
            type Output = $out;

            fn $call(self, rhs: $rhs) -> Self::Output {
                Note {
                    octave: ::core::ops::$trait::$call(self.octave, rhs.octave),
                    pitch: ::core::ops::$trait::$call(self.pitch, rhs.pitch)
                }
            }
        }

        impl<'a, P> core::ops::$trait<&'a $rhs> for &'a Note<P>
        where
            P: $crate::Notable + core::ops::$trait<Output = P>,
        {
            type Output = $out;

            fn $call(self, rhs: &'a $rhs) -> Self::Output {
                Note {
                    octave: ::core::ops::$trait::$call(self.octave, rhs.octave),
                    pitch: ::core::ops::$trait::$call(self.pitch, rhs.pitch)
                }
            }
        }
    };
    (@impl $trait:ident.$call:ident($rhs:ty)) => {
        impl_std_ops!(@impl $trait.$call($rhs) -> Note<P>);
    };
    (@impl $trait:ident.$call:ident $(-> $out:ty)?) => {
        impl_std_ops!(@impl $trait.$call(Note<P>) $(-> $out)?);
    };
    ($($trait:ident.$call:ident$(($rhs:ty))? $(-> $out:ty)?),* $(,)?) => {
        $(
            impl_std_ops!(@impl $trait.$call$(($rhs))? $(-> $out)?);
        )*
    };
}

impl_std_ops!(Add.add, Div.div, Mul.mul, Rem.rem, Sub.sub);
