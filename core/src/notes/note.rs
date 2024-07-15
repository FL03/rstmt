/*
    Appellation: note <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{IntoPitch, Octave, Pitch};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Note {
    pub(crate) octave: Octave,
    pub(crate) pitch: Pitch,
}

impl Note {
    pub fn new(pitch: Pitch) -> Self {
        Self {
            octave: Octave::default(),
            pitch,
        }
    }
    /// Returns an instance of the note's octave
    pub fn octave(&self) -> Octave {
        self.octave
    }
    /// Returns a mutable reference to the note's octave
    pub fn octave_mut(&mut self) -> &mut Octave {
        &mut self.octave
    }
    /// Returns an owned instance of the note's pitch
    pub fn pitch(&self) -> Pitch {
        self.pitch
    }
    /// Returns a mutable reference to the note's pitch
    pub fn pitch_mut(&mut self) -> &mut Pitch {
        &mut self.pitch
    }
    /// Sets the note's octave
    pub fn set_octave(&mut self, octave: Octave) {
        self.octave = octave;
    }
    /// Sets the note's pitch
    pub fn set_pitch(&mut self, pitch: Pitch) {
        self.pitch = pitch;
    }
    /// Returns a new instance of the note with the given octave
    pub fn with_octave(self, octave: Octave) -> Self {
        Self { octave, ..self }
    }
    /// Returns a new instance of the note with the given pitch
    pub fn with_pitch(self, pitch: impl IntoPitch) -> Note {
        Note {
            octave: self.octave,
            pitch: pitch.into_pitch(),
        }
    }
}

impl core::fmt::Display for Note {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}.{}", self.pitch, self.octave)
    }
}

unsafe impl Send for Note {}

unsafe impl Sync for Note {}

macro_rules! impl_std_ops {
    (@impl $trait:ident.$call:ident($rhs:ty) -> $out:ty) => {
        impl core::ops::$trait<$rhs> for Note {
            type Output = $out;

            fn $call(self, rhs: $rhs) -> Self::Output {
                Note {
                    octave: ::core::ops::$trait::$call(self.octave, rhs.octave),
                    pitch: ::core::ops::$trait::$call(self.pitch, rhs.pitch)
                }
            }
        }

        impl<'a> core::ops::$trait<&'a $rhs> for Note {
            type Output = $out;

            fn $call(self, rhs: &'a $rhs) -> Self::Output {
                Note {
                    octave: ::core::ops::$trait::$call(self.octave, rhs.octave),
                    pitch: ::core::ops::$trait::$call(self.pitch, rhs.pitch)
                }
            }
        }

        impl<'a> core::ops::$trait<$rhs> for &'a Note {
            type Output = $out;

            fn $call(self, rhs: $rhs) -> Self::Output {
                Note {
                    octave: ::core::ops::$trait::$call(self.octave, rhs.octave),
                    pitch: ::core::ops::$trait::$call(self.pitch, rhs.pitch)
                }
            }
        }

        impl<'a> core::ops::$trait<&'a $rhs> for &'a Note {
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
        impl_std_ops!(@impl $trait.$call($rhs) -> Note);
    };
    (@impl $trait:ident.$call:ident $(-> $out:ty)?) => {
        impl_std_ops!(@impl $trait.$call(Note) $(-> $out)?);
    };
    ($($trait:ident.$call:ident$(($rhs:ty))? $(-> $out:ty)?),* $(,)?) => {
        $(
            impl_std_ops!(@impl $trait.$call$(($rhs))? $(-> $out)?);
        )*
    };
}

impl_std_ops!(Add.add, Div.div, Mul.mul, Rem.rem, Sub.sub);

macro_rules! impl_binop_assign {
    (@impl $name:ident::$trait:ident.$call:ident) => {};
}

macro_rules! operator {
    (@base $trait:ident.$call:ident($lhs:ty, $rhs:ty) -> $out:ty {$($rest:tt)*}) => {
        impl core::ops::$trait<$rhs> for $lhs {
            type Output = $out;

            fn $call(self, rhs: $rhs) -> Self::Output {
                $($rest)*
            }
        }
    };
    (@a $trait:ident.$call:ident($lhs:ty, $rhs:ty) -> $out:ty {$e:expr}) => {
        operator!(@base $trait.$call($lhs, $rhs) -> $out {$e($lhs, $rhs)});
    };

    (@impl $trait:ident.$call:ident($lhs:ty, $rhs:ty) -> $out:ty {$e:expr}) => {
        impl core::ops::$trait<$rhs> for $lhs {
            type Output = $out;

            fn $call(self, rhs: $rhs) -> Self::Output {
                $e(self, rhs)
            }
        }

        impl<'a> core::ops::$trait<&'a $rhs> for $lhs {
            type Output = $out;

            fn $call(self, rhs: &'a $rhs) -> Self::Output {
                $e(self, *rhs)
            }
        }

        impl<'a> core::ops::$trait<$rhs> for &'a $lhs {
            type Output = $out;

            fn $call(self, rhs: $rhs) -> Self::Output {
                $e(*self, rhs)
            }
        }

        impl<'a> core::ops::$trait<&'a $rhs> for &'a $lhs {
            type Output = $out;

            fn $call(self, rhs: &'a $rhs) -> Self::Output {
                $e(*self, *rhs)
            }
        }
    };
    ($($trait:ident.$call:ident(self: $lhs:ty, rhs: $rhs:ty) -> $out:ty {$e:expr}),* $(,)?) => {
        $(
            operator!(@impl $trait.$call($lhs, $rhs) -> $out {$e});
        )*
    };
}
