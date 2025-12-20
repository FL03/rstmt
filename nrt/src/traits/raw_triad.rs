/*
    Appellation: raw_triad <module>
    Created At: 2025.12.20:10:36:11
    Contrib: @FL03
*/

/// [`TriadKind`] is a trait that represents the kind of a triad in Neo-Riemannian theory.
pub trait TriadKind: 'static + Copy + Send + Sync + core::fmt::Debug + core::fmt::Display {
    private!();

    fn new() -> Self;
}

/// [`RawTriadStore`] defines an interface for compatible representations of triads enabling
/// the [`TriadBase`] instance to be generic over its _container_.
pub trait RawTriadStore {
    type Elem;

    private! {}
    /// returns a reference to the root note of the triad.
    fn root(&self) -> &Self::Elem;
    /// returns a reference to the third note of the triad.
    fn third(&self) -> &Self::Elem;
    /// returns a reference to the fifth note of the triad.
    fn fifth(&self) -> &Self::Elem;
}
pub trait RawTriadStoreMut: RawTriadStore {
    /// returns a mutable reference to the root note of the triad.
    fn root_mut(&mut self) -> &mut Self::Elem;
    /// returns a mutable reference to the third note of the triad.
    fn third_mut(&mut self) -> &mut Self::Elem;
    /// returns a mutable reference to the fifth note of the triad.
    fn fifth_mut(&mut self) -> &mut Self::Elem;
    /// update the root note of the triad and return a mutable reference to the store
    fn set_root(&mut self, root: Self::Elem) -> &mut Self {
        *self.root_mut() = root;
        self
    }
    /// update the third note of the triad and return a mutable reference to the store
    fn set_third(&mut self, third: Self::Elem) -> &mut Self {
        *self.third_mut() = third;
        self
    }
    /// update the fifth note of the triad and return a mutable reference to the store
    fn set_fifth(&mut self, fifth: Self::Elem) -> &mut Self {
        *self.fifth_mut() = fifth;
        self
    }
}

/// The [`RawTriad`] trait defines the interface for all implementations of triads.
pub trait RawTriad<T>
where
    Self::Store<T>: RawTriadStore<Elem = T>,
{
    /// the type of the item stored in the triad.
    type Store<U>: RawTriadStore<Elem = U>;

    private! {}

    fn store(&self) -> &Self::Store<T>;

    fn store_mut(&mut self) -> &mut Self::Store<T>;
    /// returns a reference to the root note of the triad.
    fn root(&self) -> &T {
        self.store().root()
    }
    /// returns a mutable reference to the root note of the triad.
    fn root_mut(&mut self) -> &mut T
    where
        Self::Store<T>: RawTriadStoreMut,
    {
        self.store_mut().root_mut()
    }
    /// returns a reference to the third note of the triad.
    fn third(&self) -> &T {
        self.store().third()
    }
    /// returns a mutable reference to the third note of the triad.
    fn third_mut(&mut self) -> &mut T
    where
        Self::Store<T>: RawTriadStoreMut,
    {
        self.store_mut().third_mut()
    }
    /// returns a reference to the fifth note of the triad.
    fn fifth(&self) -> &T {
        self.store().fifth()
    }
    /// returns a mutable reference to the fifth note of the triad.
    fn fifth_mut(&mut self) -> &mut T
    where
        Self::Store<T>: RawTriadStoreMut,
    {
        self.store_mut().fifth_mut()
    }
}

/*
 ************* Implementations *************
*/

impl<T> RawTriadStore for (T, T, T) {
    type Elem = T;

    seal! {}

    fn root(&self) -> &T {
        &self.0
    }

    fn third(&self) -> &T {
        &self.1
    }

    fn fifth(&self) -> &T {
        &self.2
    }
}

impl<T> RawTriadStoreMut for (T, T, T) {
    fn root_mut(&mut self) -> &mut T {
        &mut self.0
    }
    fn third_mut(&mut self) -> &mut T {
        &mut self.1
    }

    fn fifth_mut(&mut self) -> &mut T {
        &mut self.2
    }
}

impl<T> RawTriadStore for [T; 3] {
    type Elem = T;

    seal! {}

    fn root(&self) -> &T {
        &self[0]
    }

    fn third(&self) -> &T {
        &self[1]
    }

    fn fifth(&self) -> &T {
        &self[2]
    }
}

impl<T> RawTriadStoreMut for [T; 3] {
    fn root_mut(&mut self) -> &mut T {
        &mut self[0]
    }

    fn third_mut(&mut self) -> &mut T {
        &mut self[1]
    }

    fn fifth_mut(&mut self) -> &mut T {
        &mut self[2]
    }
}

impl<T> RawTriad<T> for (T, T, T) {
    type Store<U> = (U, U, U);

    seal! {}

    fn store(&self) -> &Self::Store<T> {
        self
    }

    fn store_mut(&mut self) -> &mut Self::Store<T> {
        self
    }
}

impl<T> RawTriad<T> for [T; 3] {
    type Store<U> = [U; 3];

    seal! {}

    fn store(&self) -> &Self::Store<T> {
        self
    }

    fn store_mut(&mut self) -> &mut Self::Store<T> {
        self
    }
}
