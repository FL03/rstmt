/*
    appellation: qualities <module>
    authors: @FL03
*/

/// [`RawQuality`] is a private marker trait used to represent an intervallic quality.
pub trait RawQuality: 'static + Send + Sync + core::fmt::Debug + core::fmt::Display {
    private! {}
}

pub trait Quality: RawQuality {
    /// returns the name of the current quality
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
