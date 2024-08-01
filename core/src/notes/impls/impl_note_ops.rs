/*
    Appellation: macros <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::intervals::{Fifth, Intervals, IntoInterval, Seventh, Third};
use crate::{Note, Octave};

impl Note {
    pub fn add_major_third(&self) -> Self {
        self.add_interval(Third::Major)
    }

    pub fn sub_major_third(&self) -> Self {
        self.sub_interval(Third::Major)
    }

    pub fn add_minor_third(&self) -> Self {
        self.add_interval(Third::Minor)
    }

    pub fn sub_minor_third(&self) -> Self {
        self.sub_interval(Third::Minor)
    }

    pub fn add_perfect_fifth(&self) -> Self {
        self.add_interval(Fifth::Perfect)
    }

    pub fn sub_perfect_fifth(&self) -> Self {
        self.sub_interval(Fifth::Perfect)
    }

    pub fn add_augmented_fifth(&self) -> Self {
        self.add_interval(Fifth::Augmented)
    }

    pub fn sub_augmented_fifth(&self) -> Self {
        self.sub_interval(Fifth::Augmented)
    }

    pub fn add_diminished_fifth(&self) -> Self {
        self.add_interval(Fifth::Diminished)
    }

    pub fn sub_diminished_fifth(&self) -> Self {
        self.sub_interval(Fifth::Diminished)
    }

    pub fn add_interval<I>(&self, interval: I) -> Self
    where
        I: IntoInterval,
    {
        self + interval.into_interval()
    }

    pub fn sub_interval<I>(&self, interval: I) -> Self
    where
        I: IntoInterval,
    {
        self - interval.into_interval()
    }

    pub fn add_octave(&self, octave: Octave) -> Self {
        Self {
            octave: self.octave + octave,
            pitch: self.pitch,
        }
    }

    pub fn sub_octave(&self, octave: Octave) -> Self {
        Self {
            octave: self.octave - octave,
            pitch: self.pitch,
        }
    }
}

macro_rules! impl_interval_ops {
    (@assign $trait:ident.$call:ident) => {
        paste::paste! {
            impl core::ops::[<$trait Assign>]<$crate::Intervasl> for $crate::Interval {
                fn [<$call _assign>](&mut self, rhs: $crate::Intervals) {
                    self = $crate::Interval::from(::core::ops::$trait::$call(self.pitch, rhs.value()));
                }
            }
        }
    };
    (@note $trait:ident.$call:ident) => {
        impl core::ops::$trait<$crate::Intervals> for Note {
            type Output = Note;

            fn $call(self, rhs: $crate::Intervals) -> Self::Output {
                Note {
                    octave: self.octave,
                    pitch: core::ops::$trait::$call(self.pitch, rhs.value())
                }
            }
        }

        impl<'a> core::ops::$trait<$crate::Intervals> for &'a Note {
            type Output = Note;

            fn $call(self, rhs: $crate::Intervals) -> Self::Output {
                Note {
                    octave: self.octave,
                    pitch: ::core::ops::$trait::$call(self.pitch, rhs.value())
                }
            }
        }
    };
    (@inter $trait:ident.$call:ident) => {
        impl core::ops::$trait<Note> for $crate::Intervals {
            type Output = Note;

            fn $call(self, rhs: Note) -> Self::Output {
                Note {
                    octave: rhs.octave,
                    pitch: ::core::ops::$trait::$call(rhs.pitch, self.value())
                }
            }
        }

        impl<'a> core::ops::$trait<&'a Note> for $crate::Intervals {
            type Output = Note;

            fn $call(self, rhs: &'a Note) -> Self::Output {
                Note {
                    octave: rhs.octave,
                    pitch: ::core::ops::$trait::$call(rhs.pitch, self.value())
                }
            }
        }

        impl<'a> core::ops::$trait<Note> for &'a $crate::Intervals {
            type Output = Note;

            fn $call(self, rhs: Note) -> Self::Output {
                Note {
                    octave: rhs.octave,
                    pitch: ::core::ops::$trait::$call(rhs.pitch, self.value())
                }
            }
        }

        impl<'a> core::ops::$trait<&'a Note> for &'a $crate::Intervals {
            type Output = Note;

            fn $call(self, rhs: &'a Note) -> Self::Output {
                Note {
                    octave: rhs.octave,
                    pitch: ::core::ops::$trait::$call(rhs.pitch, self.value())
                }
            }
        }
    };
    ($($trait:ident.$call:ident),* $(,)?) => {
        $(
            impl_interval_ops!(@note $trait.$call);
            impl_interval_ops!(@inter $trait.$call);
        )*
    };
}

macro_rules! impl_std_ops {
    (@assign $trait:ident.$call:ident) => {
        paste::paste! {
            impl core::ops::[<$trait Assign>]<Note> for Note {
                fn [<$call _assign>](&mut self, rhs: Note) {
                    self.pitch = ::core::ops::$trait::$call(self.pitch, rhs.pitch);
                }
            }

            impl core::ops::[<$trait Assign>]<$crate::Intervals> for Note {
                fn [<$call _assign>](&mut self, rhs: $crate::Intervals) {
                    self.pitch = ::core::ops::$trait::$call(self.pitch, rhs.as_pitch());
                }
            }
        }
    };
    (@impl $trait:ident.$call:ident) => {
        impl core::ops::$trait<Note> for Note {
            type Output = Note;

            fn $call(self, rhs: Note) -> Self::Output {
                Note {
                    octave: ::core::ops::$trait::$call(self.octave, rhs.octave),
                    pitch: ::core::ops::$trait::$call(self.pitch, rhs.pitch)
                }
            }
        }

        impl<'a> core::ops::$trait<&'a Note> for Note {
            type Output = Note;

            fn $call(self, rhs: &'a Note) -> Self::Output {
                Note {
                    octave: ::core::ops::$trait::$call(self.octave, rhs.octave),
                    pitch: ::core::ops::$trait::$call(self.pitch, rhs.pitch)
                }
            }
        }

        impl<'a> core::ops::$trait<Note> for &'a Note {
            type Output = Note;

            fn $call(self, rhs: Note) -> Self::Output {
                Note {
                    octave: ::core::ops::$trait::$call(self.octave, rhs.octave),
                    pitch: ::core::ops::$trait::$call(self.pitch, rhs.pitch)
                }
            }
        }

        impl<'a> core::ops::$trait<&'a Note> for &'a Note {
            type Output = Note;

            fn $call(self, rhs: &'a Note) -> Self::Output {
                Note {
                    octave: ::core::ops::$trait::$call(self.octave, rhs.octave),
                    pitch: ::core::ops::$trait::$call(self.pitch, rhs.pitch)
                }
            }
        }
    };

    ($($trait:ident.$call:ident),* $(,)?) => {
        $(
            impl_std_ops!(@assign $trait.$call);
            impl_std_ops!(@impl $trait.$call);
            impl_interval_ops!($trait.$call);
        )*
    };
}

impl_std_ops!(Add.add, Div.div, Mul.mul, Rem.rem, Sub.sub);
