/*
    appellation: qualities <module>
    authors: @FL03
*/

pub trait IntervalQuality: 'static + Send + Sync + core::fmt::Debug + core::fmt::Display {
    /// Returns the quality of the interval.
    fn quality(&self) -> &str;
}
