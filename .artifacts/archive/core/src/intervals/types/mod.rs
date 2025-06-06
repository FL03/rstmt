/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{level::*, qualities::*};

pub(crate) mod level;
pub(crate) mod qualities;

#[allow(unused_imports)]
#[doc(hidden)]
pub(crate) mod prelude {
    pub use super::level::IntervalLevel;
    // pub use super::qualities::*;
}
