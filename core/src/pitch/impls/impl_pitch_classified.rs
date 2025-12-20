/*
    Appellation: impl_pitch_classified <module>
    Created At: 2025.12.20:10:02:33
    Contrib: @FL03
*/
use crate::freq::Frequency;
use crate::pitch::ClassifiedPitch;
use num_traits::{Float, FromPrimitive};
use rstmt_traits::PitchMod;

impl<T> ClassifiedPitch<T> {
    /// initialize a new instance of a Pitch
    pub const fn new(class: isize, freq: Frequency<T>) -> Self {
        Self { class, freq }
    }
    /// returns a new pitch automatically classified from the given frequency and (optional) scale
    pub fn from_freq_with_scale(freq: Frequency<T>, scale: Option<T>) -> Option<Self>
    where
        T: Float + FromPrimitive,
    {
        let class = freq.classify_by(scale)?.pmod();
        Some(Self::new(class, freq))
    }
    /// returns a copy to the index of the note's class
    pub const fn class(&self) -> isize {
        self.class
    }
    /// returns a mutable reference to the index of the note's class
    pub const fn class_mut(&mut self) -> &mut isize {
        &mut self.class
    }
    /// returns a reference to the frequency of the pitch
    pub const fn frequency(&self) -> &Frequency<T> {
        &self.freq
    }
    /// returns a mutable reference to the current octave
    pub const fn frequency_mut(&mut self) -> &mut Frequency<T> {
        &mut self.freq
    }
    /// set the pitch class and return a mutable reference to the current instance
    pub fn set_class(&mut self, class: isize) {
        self.class = class.pmod()
    }
    /// set the frequency and return a mutable reference to the current instance
    pub fn set_frequency(&mut self, freq: Frequency<T>)
    where
        T: Float + FromPrimitive,
    {
        if let Some(cls) = freq.classify_by(None) {
            self.class = cls.pmod() as isize;
            self.freq = freq;
        } else {
            panic!("Unable to classify the given frequency");
        }
    }
    /// consumes the current instance to create another with the given pitch class
    pub fn with_class(self, class: isize) -> Self {
        Self { class, ..self }
    }
    /// consumes the current instance to create another with the given frequency
    ///
    /// # Saftey
    ///
    /// The function is unsafe because it is up to the caller to ensure the class is unchanged
    /// or get updated as the method does not perform any checks to validating the frequency
    /// against the class.
    pub unsafe fn with_frequency<T2>(self, freq: Frequency<T2>) -> ClassifiedPitch<T2> {
        ClassifiedPitch {
            class: self.class,
            freq,
        }
    }
}

impl<T: core::fmt::Display> core::fmt::Display for ClassifiedPitch<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}.{}", self.class, self.freq)
    }
}

impl<T: PartialEq> PartialEq<Frequency<T>> for ClassifiedPitch<T> {
    fn eq(&self, other: &Frequency<T>) -> bool {
        self.frequency() == other
    }
}

impl<T> PartialEq<isize> for ClassifiedPitch<T> {
    fn eq(&self, other: &isize) -> bool {
        self.class() == *other
    }
}

impl<T> PartialEq<ClassifiedPitch<T>> for isize {
    fn eq(&self, other: &ClassifiedPitch<T>) -> bool {
        *self == other.class()
    }
}

impl<T> PartialOrd<isize> for ClassifiedPitch<T> {
    fn partial_cmp(&self, other: &isize) -> Option<core::cmp::Ordering> {
        self.class().partial_cmp(other)
    }
}

impl<T> PartialOrd<ClassifiedPitch<T>> for isize {
    fn partial_cmp(&self, other: &ClassifiedPitch<T>) -> Option<core::cmp::Ordering> {
        self.partial_cmp(&other.class())
    }
}
