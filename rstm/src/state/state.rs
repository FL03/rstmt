/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(transparent)]
pub struct State<T = String>(pub(crate) T);

impl<T> State<T> {
    pub fn new(state: T) -> Self {
        Self(state)
    }

    pub fn into_inner(self) -> T {
        self.0
    }

    pub fn as_ref(&self) -> &T {
        &self.0
    }

    pub fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> AsRef<T> for State<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for State<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

unsafe impl<T> Send for State<T> where T: Send {}

unsafe impl<T> Sync for State<T> where T: Sync {}

impl<T> core::borrow::Borrow<T> for State<T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T> core::borrow::BorrowMut<T> for State<T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> core::ops::Deref for State<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> core::ops::DerefMut for State<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> core::fmt::Display for State<T>
where
    T: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> From<T> for State<T> {
    fn from(state: T) -> Self {
        Self(state)
    }
}
