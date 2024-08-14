/*
    Appellation: builder <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(unused)]
use super::{Triad, Triads};
use rstmt::Note;

#[doc(hidden)]
pub struct TriadBuilder {
    pub class: Triads,
    pub root: Option<Note>,
    pub third: Option<Note>,
    pub fifth: Option<Note>,
}

impl TriadBuilder {
    pub fn new(class: Triads) -> Self {
        Self {
            class,
            root: None,
            third: None,
            fifth: None,
        }
    }

    pub fn with_root(mut self, root: Note) -> Self {
        self.root = Some(root);
        self
    }

    pub fn with_third(mut self, third: Note) -> Self {
        self.third = Some(third);
        self
    }

    pub fn with_fifth(mut self, fifth: Note) -> Self {
        self.fifth = Some(fifth);
        self
    }

    pub fn build<K>(self) -> Result<Triad<K>, &'static str> {
        let root = self.root.ok_or("Root note is required")?;
        let third = self.third.ok_or("Third note is required")?;
        let fifth = self.fifth.ok_or("Fifth note is required")?;

        unimplemented!()
    }
}
