/*
    Appellation: classes <module>
    Created At: 2025.12.20:10:03:55
    Contrib: @FL03
*/
macro_rules! classes {
    (@impl $name:ident::<Natural>) => {
        paste::paste! {
            pub type $name = $crate::pitch::PitchClass<$crate::pitch::[<$name Note>], $crate::pitch::Natural>;
        }
    };
    (@impl $name:ident::<$kind:ident>) => {
        paste::paste! {
            pub type [<$name $kind>] = $crate::pitch::PitchClass<$crate::pitch::[<$name $kind Note>], $crate::pitch::$kind>;
        }
    };
    (@impl $name:ident::<$($kind:ident),+ $(,)?>) => {
        $(classes! { @impl $name::<$kind> })*
    };
    ($($name:ident::<$($K:ident),* $(,)?>),* $(,)?) => {
        $(classes! { @impl $name::<Natural, $($K),*> })*
    };
}

classes! {
    C::<Sharp>,
    D::<Flat, Sharp>,
    E::<Flat>,
    F::<Sharp>,
    G::<Flat, Sharp>,
    A::<Flat, Sharp>,
    B::<Flat>,
}
