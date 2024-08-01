/*
    Appellation: macros <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::Note;

macro_rules! impl_interval_ops {
    (@assign $trait:ident.$call:ident) => {
        paste::paste! {
            impl core::ops::[<$trait Assign>]<$crate::Interval> for $crate::Interval {
                fn [<$call _assign>](&mut self, rhs: $crate::Interval) {
                    self = $crate::Interval::from(::core::ops::$trait::$call(self.pitch, rhs.value()));
                }
            }
        }
    };
    (@note $trait:ident.$call:ident) => {
        impl core::ops::$trait<$crate::Interval> for Note {
            type Output = Note;

            fn $call(self, rhs: $crate::Interval) -> Self::Output {
                Note {
                    octave: self.octave,
                    pitch: core::ops::$trait::$call(self.pitch, rhs.value())
                }
            }
        }

        impl<'a> core::ops::$trait<$crate::Interval> for &'a Note {
            type Output = Note;

            fn $call(self, rhs: $crate::Interval) -> Self::Output {
                Note {
                    octave: self.octave,
                    pitch: ::core::ops::$trait::$call(self.pitch, rhs.value())
                }
            }
        }
    };
    (@inter $trait:ident.$call:ident) => {
        impl core::ops::$trait<Note> for $crate::Interval {
            type Output = Note;

            fn $call(self, rhs: Note) -> Self::Output {
                Note {
                    octave: rhs.octave,
                    pitch: ::core::ops::$trait::$call(rhs.pitch, self.value())
                }
            }
        }

        impl<'a> core::ops::$trait<&'a Note> for $crate::Interval {
            type Output = Note;

            fn $call(self, rhs: &'a Note) -> Self::Output {
                Note {
                    octave: rhs.octave,
                    pitch: ::core::ops::$trait::$call(rhs.pitch, self.value())
                }
            }
        }

        impl<'a> core::ops::$trait<Note> for &'a $crate::Interval {
            type Output = Note;

            fn $call(self, rhs: Note) -> Self::Output {
                Note {
                    octave: rhs.octave,
                    pitch: ::core::ops::$trait::$call(rhs.pitch, self.value())
                }
            }
        }

        impl<'a> core::ops::$trait<&'a Note> for &'a $crate::Interval {
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

            impl core::ops::[<$trait Assign>]<$crate::Interval> for Note {
                fn [<$call _assign>](&mut self, rhs: $crate::Interval) {
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
