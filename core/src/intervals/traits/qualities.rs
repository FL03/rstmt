/*
    appellation: qualities <module>
    authors: @FL03
*/

/// [`RawQuality`] is used to represent the _quality_ of an interval.
pub trait RawQuality: 'static + Send + Sync + core::fmt::Debug + core::fmt::Display {
    private! {}
}

pub trait Quality: RawQuality {
    /// returns the quality of the interval.
    fn name(&self) -> &str;
}

/*
 ************* Implementations *************
*/
#[allow(unused_macros)]
macro_rules! impl_raw_quality {
    (@impl $name:ident::<$T:ty>) => {
        impl RawQuality for $T {
            seal! {}
        }
    };
    ($($name:ident: $T:ty),* $(,)?) => {
        $(impl_raw_quality! { @impl $name::<$T> })*
    };
}
