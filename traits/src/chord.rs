/*
    Appellation: chord <module>
    Created At: 2025.12.23:17:32:04
    Contrib: @FL03
*/
/// The [`RawChord`] trait works to define a basic interface shared by all compatible
/// reprsentations of a chord. Since a chord is essentially a sequence of pitches, the trait
/// captures this behavior through association with an element type.
pub trait RawChord {
    type Elem;
}

pub trait ChordRepr: RawChord {
    /// returns the number of elements in the chord representation.
    fn len(&self) -> usize;
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
        core::cell::Cell<T>,
        core::cell::OnceCell<T>,
        core::cell::RefCell<T>,
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

macro_rules! impl_raw_chord_tuple {
    (@impl<$T:ident> ($($name:ident),+ $(,)?)) => {
        impl<$T> RawChord for ($($name),+) {
            type Elem = $T;
        }
    };
    (impl<$T:ident> {$(($($name:ident),+)),* $(,)?}) => {
        $(impl_raw_chord_tuple! { @impl<$T> ($($name),+) } )*
    };
}

impl_raw_chord_tuple! {
    impl<T> {
        (T, T),
        (T, T, T),
        (T, T, T, T),
        (T, T, T, T, T),
        (T, T, T, T, T, T),
        (T, T, T, T, T, T, T),
        (T, T, T, T, T, T, T, T),
        (T, T, T, T, T, T, T, T, T),
        (T, T, T, T, T, T, T, T, T, T),
    }
}

impl<T> ChordRepr for [T] {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> ChordRepr for &[T] {
    fn len(&self) -> usize {
        (*self).len()
    }
}

impl<T> ChordRepr for &mut [T] {
    fn len(&self) -> usize {
        (**self).len()
    }
}

#[cfg(feature = "alloc")]
impl<T> ChordRepr for alloc::vec::Vec<T> {
    fn len(&self) -> usize {
        self.len()
    }
}
