/*
    Appellation: units <module>
    Created At: 2025.12.23:16:01:01
    Contrib: @FL03
*/

#[allow(unused_macros)]
/// The [`unit_type`] macro works to facilitate the creation of so-called unit types, i.e. 
/// either an enum without any variants or a struct without any fields. Such types
/// are useful in a variety of contexts, such as type-level programming, marker types,
/// and zero-sized types.
macro_rules! unit_type {
    (@impl $(#[$meta:meta])* $vis:vis enum $name:ident $({})? $(;)?) => {
        $(#[$meta])*
        $vis enum $name {}
    };
    (@impl $(#[$meta:meta])* $vis:vis struct $name:ident $(;)?) => {
        $(#[$meta])* #[derive(Default)]
        $vis struct $name;
    };
    ($($(#[$meta:meta])* $vis:vis $type:ident $name:ident);* $(;)?) => {
        $(unit_type! { @impl $(#[$meta])* $vis $type $name })*
    };
}
