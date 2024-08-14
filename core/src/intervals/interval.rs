/*
    Appellation: interval <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub struct Interval<Q> {
    pub level: IntervalLevel,
    pub quality: Option<Q>,
    pub value: i8,
}

pub enum IntervalLevel {
    Semitone,
    Tone,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Octave,
}

pub trait IntervalQuality {
    fn level(&self) -> IntervalLevel;
    
    fn name(&self) -> &str;

    fn value(&self) -> i8;
}
