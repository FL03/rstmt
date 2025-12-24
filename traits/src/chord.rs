/*
    Appellation: chord <module>
    Created At: 2025.12.23:17:32:04
    Contrib: @FL03
*/

pub trait RawChord {
    type Elem;
}

/*
 ************* Implementations *************
*/

impl<C, T> RawChord for &C
where
    C: RawChord<Elem = T>,
{
    type Elem = C::Elem;
}

impl<C, T> RawChord for &mut C
where
    C: RawChord<Elem = T>,
{
    type Elem = C::Elem;
}

macro_rules! impl_raw_chord  {
    (impl<Elem = $elem:ident> $trait:ident for {$(
        $($cont:ident)::*<$($T:ident),*> $({where $($rest:tt)*})?
    ),* $(,)?}) => {
        $(impl_raw_chord! {
            @impl<Elem = $elem> $trait for $($cont)::*<$($T),*> $(where $($rest)*)?
        })*
    };
    (@impl<Elem = $elem:ident> $trait:ident for $($cont:ident)::*<$($T:ident),*> $(where $($rest:tt)*)?) => {
        impl<$($T),*> $trait for $($cont)::*<$($T),*> $(where $($rest)*)? {
            type Elem = $elem;
        }
    };
}

impl_raw_chord! {
    impl<Elem = T> RawChord for {
        core::option::Option<T>,
        core::cell::UnsafeCell<T>,
        core::ops::Range<T>,
        core::result::Result<T, E>,
    }
}

#[cfg(feature = "alloc")]
impl_raw_chord! {
    impl<Elem = T> RawChord for {
        alloc::boxed::Box<T>,
        alloc::rc::Rc<T>,
        alloc::sync::Arc<T>,
        alloc::vec::Vec<T>,
        alloc::collections::BTreeSet<T>,
        alloc::collections::LinkedList<T>,
        alloc::collections::VecDeque<T>,
        alloc::collections::BinaryHeap<T>,
        alloc::collections::BTreeMap<K, T>,
    }
}

#[cfg(feature = "std")]
impl_raw_chord! {
    impl<Elem = T> RawChord for {
        std::cell::Cell<T>,
        std::cell::OnceCell<T>,
        std::cell::RefCell<T>,
        std::sync::Mutex<T>,
        std::sync::RwLock<T>,
        std::sync::LazyLock<T>,
        std::collections::HashMap<K, T>,
        std::collections::HashSet<T>,
    }
}

#[cfg(feature = "hashbrown")]
impl_raw_chord! {
    impl<Elem = T> RawChord for {
        hashbrown::HashMap<K, T, S>,
        hashbrown::HashSet<T, S>,
    }
}

impl<T> RawChord for [T] {
    type Elem = T;
}

impl<T> RawChord for &[T] {
    type Elem = T;
}

impl<T> RawChord for &mut [T] {
    type Elem = T;
}

impl<const N: usize, T> RawChord for [T; N] {
    type Elem = T;
}

impl<T> RawChord for (T, T, T) {
    type Elem = T;
}
