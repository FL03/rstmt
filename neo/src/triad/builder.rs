/*
    Appellation: builder <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use rstmt::Note;

pub struct TriadBuilder {
    pub root: Option<Note>,
    pub third: Option<Note>,
    pub fifth: Option<Note>,
}
