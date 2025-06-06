/*
    Appellation: pure <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{frequency::Frequency, tone::PureTone};

pub(crate) mod frequency;
pub(crate) mod tone;

pub(crate) mod prelude {
    pub use super::frequency::Frequency;
    pub use super::tone::PureTone;
}
