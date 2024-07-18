/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

impl<T> From<T> for State<T> {
    fn from(state: T) -> Self {
        Self(state)
    }
}
